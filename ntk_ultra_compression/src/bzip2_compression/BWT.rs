//by litissia.ben-mohand
/*
    Performs the Burrows-Wheeler Transform (BWT) on the input data.
    This transformation is used to group similar characters together.
 */
pub fn bwt_encode(data: &[u8]) -> (Vec<u8>, usize)
{
    let len = data.len();
    let mut rotations: Vec<Vec<u8>> = Vec::with_capacity(len);//matrice len*len
    
    // Generate all cyclic rotations of the input data
    for i in 0..len
    {
        let mut rotation = Vec::with_capacity(len);
        rotation.extend_from_slice(&data[i..]);
        rotation.extend_from_slice(&data[..i]);
        rotations.push(rotation);
    }

    //sort the rotations lexicographically
    rotations.sort();

    //extract the last column
    let last_col : Vec<u8> = rotations.iter().map(|r| r[len-1]).collect();

    //find the index of the original input data in the sorted rotations
    let original_indx = rotations.iter().position(|r| r == data).unwrap();
    
    (last_col,original_indx)
}



/*
    Decodes a Burrows-Wheeler Transformed (BWT) input back to its original form.
 */
pub fn bwt_decode(bwt_output: &[u8], original_index: usize) -> Vec<u8>
{
    let len = bwt_output.len();

    // Create the first column by sorting the transformed data
    let mut first_col = bwt_output.to_vec();
    first_col.sort();

    // Histogram for tracking character occurrences
    let mut histo = vec![0; 256];

    // Build the mapping from last column to first column
    let mut last_to_first = vec![0; len];
    for (i,&c) in bwt_output.iter().enumerate()
    {
        let mut count_occ = 0;
        for (j,&fc) in first_col.iter().enumerate()
        {
            if fc == c 
            {
                if count_occ == histo[c as usize]
                {
                    last_to_first[i] = j;
                    break;
                }
                count_occ +=1;
            }
        }
        histo[c as usize] += 1;
    }

    // Reconstruct the original data using the mapping
    let mut original = vec![0; len];
    let mut indx = original_index;

    for i in(0..len).rev()
    {
        original[i] = bwt_output[indx];
        indx = last_to_first[indx];
    }
    original
}
