import { useState, useCallback } from 'react';
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
} from '@mui/material';
import {
  Folder as FolderIcon,
  Save as SaveIcon,
  Compress as CompressIcon,
  Unarchive as UnarchiveIcon,
} from '@mui/icons-material';

interface CompressionOptions {
  level: number;
  threads: number;
  block_size: number;
  dictionary_size: number;
  use_encryption: boolean;
  password?: string;
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
  const [compressionLevel, setCompressionLevel] = useState(6);
  const [isProcessing, setIsProcessing] = useState(false);
  const [metadata, setMetadata] = useState<FileMetadata | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isCompressed, setIsCompressed] = useState(false);

  const handleError = useCallback((e: any) => {
    console.error('Operation failed:', e);
    setError(typeof e === 'string' ? e : e.message || 'An unknown error occurred');
    setIsProcessing(false);
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

    try {
      const options: CompressionOptions = {
        level: compressionLevel,
        threads: navigator.hardwareConcurrency || 4,
        block_size: 1024 * 1024, // 1MB
        dictionary_size: 32 * 1024 * 1024, // 32MB
        use_encryption: useEncryption,
        password: useEncryption ? password : undefined,
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
    }
  };

  const handleDecompress = async () => {
    if (!inputPath || !outputPath) {
      setError('Please select input and output files');
      return;
    }

    setIsProcessing(true);
    setError(null);

    try {
      await invoke<void>('decompress', {
        request: {
          input_path: inputPath,
          output_path: outputPath,
          password: useEncryption ? password : null,
        },
      });

      setMetadata(null);
      setError(null);
    } catch (e) {
      handleError(e);
    } finally {
      setIsProcessing(false);
    }
  };

  const formatSize = (size: number) => {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let value = size;
    let unit = 0;
    while (value >= 1024 && unit < units.length - 1) {
      value /= 1024;
      unit++;
    }
    return `${value.toFixed(2)} ${units[unit]}`;
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Container maxWidth="md" sx={{ py: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom align="center">
          NTK Ultra-Compression
        </Typography>

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

          <Box sx={{ mb: 3 }}>
            <Typography gutterBottom>Compression Level: {compressionLevel}</Typography>
            <Slider
              value={compressionLevel}
              onChange={(_, value) => setCompressionLevel(value as number)}
              min={1}
              max={9}
              marks
              disabled={isProcessing || isCompressed}
            />
          </Box>

          <Box sx={{ mb: 3 }}>
            <Typography component="div" gutterBottom>
              <Box sx={{ display: 'flex', alignItems: 'center' }}>
                <Switch
                  checked={useEncryption}
                  onChange={(e) => setUseEncryption(e.target.checked)}
                  disabled={isProcessing}
                />
                Enable Encryption
              </Box>
            </Typography>
          </Box>

          {useEncryption && (
            <Box sx={{ mb: 3 }}>
              <TextField
                type="password"
                label="Password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required={useEncryption}
                disabled={isProcessing}
                fullWidth
              />
            </Box>
          )}

          {error && (
            <Box sx={{ mb: 3 }}>
              <Alert severity="error">{error}</Alert>
            </Box>
          )}

          <Box sx={{ display: 'flex', gap: 2 }}>
            <Button
              variant="contained"
              color="primary"
              startIcon={<CompressIcon />}
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
              {isProcessing ? <CircularProgress size={24} /> : 'Compress'}
            </Button>
            <Button
              variant="contained"
              color="secondary"
              startIcon={<UnarchiveIcon />}
              onClick={handleDecompress}
              disabled={
                isProcessing ||
                !inputPath ||
                !outputPath ||
                (useEncryption && !password) ||
                !isCompressed
              }
              fullWidth
            >
              {isProcessing ? <CircularProgress size={24} /> : 'Decompress'}
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
                    <TableCell>{metadata.encrypted ? 'Yes' : 'No'}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Creation Time</TableCell>
                    <TableCell>{formatDate(metadata.creation_time)}</TableCell>
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
      </Container>
    </ThemeProvider>
  );
}

export default App; 