import { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open, save } from '@tauri-apps/api/dialog';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import {
  Box,
  Button,
  Container,
  Typography,
  Slider,
  Switch,
  TextField,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableRow,
  CircularProgress,
  Alert,
  CssBaseline,
  LinearProgress,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
  FormControlLabel,
  FormGroup,
  Tabs,
  Tab,
} from '@mui/material';
import {
  Folder as FolderIcon,
  Save as SaveIcon,
  Compress as CompressIcon,
  Unarchive as UnarchiveIcon,
  Lock as LockIcon,
  LockOpen as LockOpenIcon,
  Image as ImageIcon,
} from '@mui/icons-material';
import { listen } from '@tauri-apps/api/event';

interface CompressionOptions {
  level: number;
  threads: number;
  block_size: number;
  dictionary_size: number;
  use_encryption: boolean;
  password?: string;
  use_steganography: boolean;
  steganography_image?: string;
}

interface FileMetadata {
  original_name: string;
  original_size: number;
  compressed_size: number;
  compression_ratio: number;
  encrypted: boolean;
  creation_time: number;
  checksum: string;
}

interface ProgressEvent {
  processed_bytes: number;
  total_bytes: number;
  percent: number;
  speed_mbps: number;
  remaining_seconds: number;
}

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#90caf9',
    },
    secondary: {
      main: '#f48fb1',
    },
  },
});

function App() {
  const [inputPath, setInputPath] = useState('');
  const [outputPath, setOutputPath] = useState('');
  const [password, setPassword] = useState('');
  const [useEncryption, setUseEncryption] = useState(false);
  const [compressionLevel, setCompressionLevel] = useState(19);
  const [isProcessing, setIsProcessing] = useState(false);
  const [metadata, setMetadata] = useState<FileMetadata | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isCompressed, setIsCompressed] = useState(false);
  const [progress, setProgress] = useState<ProgressEvent | null>(null);
  const [showPasswordDialog, setShowPasswordDialog] = useState(false);
  const [tempDecompressRequest, setTempDecompressRequest] = useState<{
    input_path: string;
    output_path: string;
  } | null>(null);
  const [currentTab, setCurrentTab] = useState(0);
  const [useSteganography, setUseSteganography] = useState(false);
  const [steganographyImage, setSteganographyImage] = useState('');
  const [steganographyOutput, setSteganographyOutput] = useState('');

  const handleError = useCallback((e: any) => {
    console.error('Operation failed:', e);
    setError(typeof e === 'string' ? e : e.message || 'An unknown error occurred');
    setIsProcessing(false);
  }, []);

  useEffect(() => {
    // Configurer le gestionnaire de progression au démarrage
    invoke('set_progress_handler')
      .catch(e => console.error('Failed to set progress handler:', e));

    // Écouter les événements de progression
    const unsubscribe = listen<ProgressEvent>('progress', (event) => {
      setProgress(event.payload);
    });

    // Nettoyer lors du démontage
    return () => {
      invoke('clear_progress_handler')
        .catch(e => console.error('Failed to clear progress handler:', e));
      unsubscribe.then(fn => fn());
    };
  }, []);

  const selectInputFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'All Files', extensions: ['*'] }],
      });

      if (selected) {
        const path = selected as string;
        setInputPath(path);
        setError(null);

        // Try to detect if this is a compressed file
        try {
          const metadata = await invoke<FileMetadata>('get_metadata', { path });
          setMetadata(metadata);
          setIsCompressed(true);
          // Suggest uncompressed output name
          setOutputPath(path.replace(/\.ntk$/, ''));
        } catch {
          setIsCompressed(false);
          setMetadata(null);
          // Suggest compressed output name
          setOutputPath(`${path}.ntk`);
        }
      }
    } catch (e) {
      handleError(e);
    }
  };

  const selectOutputFile = async () => {
    try {
      const selected = await save({
        filters: [{ name: 'All Files', extensions: ['*'] }],
        defaultPath: outputPath,
      });

      if (selected) {
        setOutputPath(selected);
        setError(null);
      }
    } catch (e) {
      handleError(e);
    }
  };

  const handleCompress = async () => {
    if (!inputPath || !outputPath) {
      setError('Please select input and output files');
      return;
    }

    if (useEncryption && !password) {
      setError('Please enter a password for encryption');
      return;
    }

    setIsProcessing(true);
    setError(null);
    setProgress(null);

    try {
      const options: CompressionOptions = {
        level: compressionLevel,
        threads: navigator.hardwareConcurrency || 4,
        block_size: 16 * 1024 * 1024,
        dictionary_size: 64 * 1024 * 1024,
        use_encryption: useEncryption,
        password: useEncryption ? password : undefined,
        use_steganography: false,
      };

      const metadata = await invoke<FileMetadata>('compress', {
        request: {
          input_path: inputPath,
          output_path: outputPath,
          options,
        },
      });

      setMetadata(metadata);
      setError(null);
    } catch (e) {
      handleError(e);
    } finally {
      setIsProcessing(false);
      setProgress(null);
    }
  };

  const handleDecompress = async () => {
    if (!inputPath || !outputPath) {
      setError('Please select input and output files');
      return;
    }

    try {
      // Vérifier si le fichier est chiffré
      const metadata = await invoke<FileMetadata>('get_metadata', { path: inputPath });
      
      if (metadata.encrypted) {
        if (!password) {
          setTempDecompressRequest({ input_path: inputPath, output_path: outputPath });
          setShowPasswordDialog(true);
          return;
        }
        await startDecompression(inputPath, outputPath, password);
      } else {
        // Si le fichier n'est pas chiffré, on décompresse sans mot de passe
        await startDecompression(inputPath, outputPath, null);
      }
    } catch (e) {
      handleError(e);
    }
  };

  const startDecompression = async (input: string, output: string, pwd: string | null) => {
    setIsProcessing(true);
    setError(null);
    setProgress(null);

    try {
      await invoke<void>('decompress', {
        request: {
          input_path: input,
          output_path: output,
          password: pwd || null,
        },
      });

      setMetadata(null);
      setError(null);
      setShowPasswordDialog(false);
      setTempDecompressRequest(null);
      setPassword('');
    } catch (e) {
      handleError(e);
    } finally {
      setIsProcessing(false);
      setProgress(null);
    }
  };

  const formatSize = (bytes: number) => {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };

  const formatTime = (seconds: number) => {
    if (seconds < 60) {
      return `${Math.round(seconds)}s`;
    } else if (seconds < 3600) {
      const minutes = Math.floor(seconds / 60);
      const remainingSeconds = Math.round(seconds % 60);
      return `${minutes}m ${remainingSeconds}s`;
    } else {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return `${hours}h ${minutes}m`;
    }
  };

  const handleSteganographyHide = async () => {
    try {
      setIsProcessing(true);
      setError(null);
      await invoke('hide_in_image', {
        request: {
          archive_path: inputPath,
          image_path: steganographyImage,
          output_path: steganographyOutput,
        },
      });
      setIsProcessing(false);
    } catch (e) {
      setError(e as string);
      setIsProcessing(false);
    }
  };

  const handleSteganographyExtract = async () => {
    try {
      setIsProcessing(true);
      setError(null);
      await invoke('extract_from_image', {
        request: {
          image_path: steganographyImage,
          output_path: outputPath,
        },
      });
      setIsProcessing(false);
    } catch (e) {
      setError(e as string);
      setIsProcessing(false);
    }
  };

  const handleSteganographyImageSelect = async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg'] }],
    });
    if (selected) {
      setSteganographyImage(selected as string);
    }
  };

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Container maxWidth="md" sx={{ py: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom align="center">
          NTK Ultra-Compression
        </Typography>

        <Tabs value={currentTab} onChange={(_, newValue) => setCurrentTab(newValue)} sx={{ mb: 3 }}>
          <Tab label="Compression" />
          <Tab label="Stéganographie" />
        </Tabs>

        {error && (
          <Alert severity="error" sx={{ mb: 3 }}>
            {error}
          </Alert>
        )}

        {currentTab === 0 && (
          <>
            <Paper sx={{ p: 3, mb: 3 }}>
              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<FolderIcon />}
                  onClick={selectInputFile}
                  disabled={isProcessing}
                  fullWidth
                  sx={{ mb: 1 }}
                >
                  Select Input File
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {inputPath || 'No file selected'}
                </Typography>
              </Box>

              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<SaveIcon />}
                  onClick={selectOutputFile}
                  disabled={isProcessing}
                  fullWidth
                  sx={{ mb: 1 }}
                >
                  Select Output Location
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {outputPath || 'No location selected'}
                </Typography>
              </Box>

              {!isCompressed && (
                <>
                  <Box sx={{ mb: 3 }}>
                    <Typography gutterBottom>
                      Compression Level: {compressionLevel}
                    </Typography>
                    <Slider
                      value={compressionLevel}
                      onChange={(_, value) => setCompressionLevel(value as number)}
                      min={1}
                      max={22}
                      marks={[
                        { value: 1, label: '1' },
                        { value: 6, label: '6' },
                        { value: 12, label: '12' },
                        { value: 19, label: '19' },
                        { value: 22, label: '22' },
                      ]}
                      disabled={isProcessing}
                    />
                  </Box>

                  <Box sx={{ mb: 3 }}>
                    <FormGroup>
                      <FormControlLabel
                        control={
                          <Switch
                            checked={useEncryption}
                            onChange={(e) => setUseEncryption(e.target.checked)}
                            disabled={isProcessing}
                            icon={<LockOpenIcon />}
                            checkedIcon={<LockIcon />}
                          />
                        }
                        label="Enable Encryption"
                      />
                    </FormGroup>
                  </Box>
                </>
              )}

              {(useEncryption || (isCompressed && metadata?.encrypted)) && (
                <Box sx={{ mb: 3 }}>
                  <TextField
                    type="password"
                    label="Password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    required={useEncryption || (isCompressed && metadata?.encrypted)}
                    disabled={isProcessing}
                    fullWidth
                    error={useEncryption && !password}
                    helperText={useEncryption && !password ? "Password is required for encryption" : ""}
                    InputProps={{
                      startAdornment: metadata?.encrypted ? <LockIcon color="primary" /> : useEncryption ? <LockIcon /> : undefined,
                    }}
                  />
                </Box>
              )}

              {isProcessing && progress && (
                <Box sx={{ width: '100%', mt: 3, mb: 3 }}>
                  <Typography variant="h6" gutterBottom>
                    {isCompressed ? 'Décompression en cours...' : 'Compression en cours...'}
                  </Typography>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                    <Typography variant="body2" color="text.secondary">
                      {formatSize(progress.processed_bytes)} / {formatSize(progress.total_bytes)}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      {progress.speed_mbps.toFixed(1)} MB/s
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      {formatTime(progress.remaining_seconds)} restant
                    </Typography>
                  </Box>
                  <LinearProgress 
                    variant="determinate" 
                    value={progress.percent} 
                    sx={{
                      height: 8,
                      borderRadius: 4,
                      '& .MuiLinearProgress-bar': {
                        borderRadius: 4,
                      },
                    }}
                  />
                  <Typography 
                    variant="body2" 
                    color="text.secondary" 
                    align="center" 
                    sx={{ mt: 1 }}
                  >
                    {Math.round(progress.percent)}%
                  </Typography>
                </Box>
              )}

              <Box sx={{ display: 'flex', gap: 2 }}>
                <Button
                  variant="contained"
                  color="primary"
                  startIcon={isProcessing ? <CircularProgress size={24} /> : <CompressIcon />}
                  onClick={handleCompress}
                  disabled={
                    isProcessing ||
                    !inputPath ||
                    !outputPath ||
                    (useEncryption && !password) ||
                    isCompressed
                  }
                  fullWidth
                >
                  Compress
                </Button>
                <Button
                  variant="contained"
                  color="secondary"
                  startIcon={isProcessing ? <CircularProgress size={24} /> : <UnarchiveIcon />}
                  onClick={handleDecompress}
                  disabled={
                    isProcessing ||
                    !inputPath ||
                    !outputPath ||
                    !isCompressed ||
                    (metadata?.encrypted && !password)
                  }
                  fullWidth
                >
                  {metadata?.encrypted ? 'Decrypt and Decompress' : 'Decompress'}
                </Button>
              </Box>
            </Paper>

            {metadata && (
              <Paper sx={{ p: 3 }}>
                <Typography variant="h6" gutterBottom>
                  File Information
                </Typography>
                <TableContainer>
                  <Table>
                    <TableBody>
                      <TableRow>
                        <TableCell>File Name</TableCell>
                        <TableCell>{metadata.original_name}</TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell>Original Size</TableCell>
                        <TableCell>{formatSize(metadata.original_size)}</TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell>Compressed Size</TableCell>
                        <TableCell>{formatSize(metadata.compressed_size)}</TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell>Compression Ratio</TableCell>
                        <TableCell>{metadata.compression_ratio.toFixed(2)}x</TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell>Encrypted</TableCell>
                        <TableCell>
                          {metadata.encrypted ? (
                            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                              <LockIcon color="primary" />
                              Yes
                            </Box>
                          ) : (
                            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                              <LockOpenIcon />
                              No
                            </Box>
                          )}
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell>Creation Time</TableCell>
                        <TableCell>{new Date(metadata.creation_time * 1000).toLocaleString()}</TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell>Checksum</TableCell>
                        <TableCell
                          sx={{
                            fontFamily: 'monospace',
                            wordBreak: 'break-all',
                          }}
                        >
                          {metadata.checksum}
                        </TableCell>
                      </TableRow>
                    </TableBody>
                  </Table>
                </TableContainer>
              </Paper>
            )}
          </>
        )}

        {currentTab === 1 && (
          <>
            <Paper sx={{ p: 3, mb: 3 }}>
              <Typography variant="h6" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <ImageIcon />
                Cacher une archive dans une image
              </Typography>

              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<FolderIcon />}
                  onClick={selectInputFile}
                  disabled={isProcessing}
                  sx={{ mr: 2 }}
                >
                  Sélectionner l'archive (.ntk)
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {inputPath || 'Aucune archive sélectionnée'}
                </Typography>
              </Box>

              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<ImageIcon />}
                  onClick={handleSteganographyImageSelect}
                  disabled={isProcessing}
                  sx={{ mr: 2 }}
                >
                  Sélectionner l'image support
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {steganographyImage || 'Aucune image sélectionnée'}
                </Typography>
              </Box>

              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<SaveIcon />}
                  onClick={async () => {
                    const selected = await save({
                      filters: [{ name: 'Images PNG', extensions: ['png'] }],
                      defaultPath: steganographyOutput,
                    });
                    if (selected) {
                      setSteganographyOutput(selected);
                    }
                  }}
                  disabled={isProcessing}
                  sx={{ mr: 2 }}
                >
                  Sélectionner la destination (.png)
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {steganographyOutput || 'Aucune destination sélectionnée'}
                </Typography>
              </Box>

              <Box sx={{ display: 'flex', gap: 2 }}>
                <Button
                  variant="contained"
                  color="primary"
                  onClick={handleSteganographyHide}
                  disabled={isProcessing || !inputPath || !steganographyImage || !steganographyOutput}
                  startIcon={<ImageIcon />}
                  fullWidth
                >
                  Cacher l'archive dans l'image
                </Button>
              </Box>
            </Paper>

            <Paper sx={{ p: 3 }}>
              <Typography variant="h6" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <UnarchiveIcon />
                Extraire une archive cachée
              </Typography>

              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<ImageIcon />}
                  onClick={handleSteganographyImageSelect}
                  disabled={isProcessing}
                  sx={{ mr: 2 }}
                >
                  Sélectionner l'image contenant l'archive
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {steganographyImage || 'Aucune image sélectionnée'}
                </Typography>
              </Box>

              <Box sx={{ mb: 3 }}>
                <Button
                  variant="contained"
                  startIcon={<SaveIcon />}
                  onClick={async () => {
                    const selected = await save({
                      filters: [{ name: 'Archive NTK', extensions: ['ntk'] }],
                      defaultPath: outputPath,
                    });
                    if (selected) {
                      setOutputPath(selected);
                    }
                  }}
                  disabled={isProcessing}
                  sx={{ mr: 2 }}
                >
                  Sélectionner la destination (.ntk)
                </Button>
                <Typography variant="body2" color="text.secondary">
                  {outputPath || 'Aucune destination sélectionnée'}
                </Typography>
              </Box>

              <Box sx={{ display: 'flex', gap: 2 }}>
                <Button
                  variant="contained"
                  color="secondary"
                  onClick={handleSteganographyExtract}
                  disabled={isProcessing || !steganographyImage || !outputPath}
                  startIcon={<UnarchiveIcon />}
                  fullWidth
                >
                  Extraire l'archive de l'image
                </Button>
              </Box>
            </Paper>
          </>
        )}

        <Dialog 
          open={showPasswordDialog} 
          onClose={() => setShowPasswordDialog(false)}
        >
          <DialogTitle>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <LockIcon color="primary" />
              Password Required
            </Box>
          </DialogTitle>
          <DialogContent>
            <DialogContentText>
              This archive is encrypted. Please enter the password to decrypt it.
            </DialogContentText>
            <TextField
              autoFocus
              margin="dense"
              label="Password"
              type="password"
              fullWidth
              variant="outlined"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              error={!password}
              helperText={!password ? "Password is required" : ""}
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={() => {
              setShowPasswordDialog(false);
              setPassword('');
              setTempDecompressRequest(null);
            }}>
              Cancel
            </Button>
            <Button 
              onClick={() => {
                if (tempDecompressRequest) {
                  startDecompression(
                    tempDecompressRequest.input_path,
                    tempDecompressRequest.output_path,
                    password
                  );
                }
              }} 
              disabled={!password}
              variant="contained"
            >
              Decrypt and Decompress
            </Button>
          </DialogActions>
        </Dialog>
      </Container>
    </ThemeProvider>
  );
}

export default App; 