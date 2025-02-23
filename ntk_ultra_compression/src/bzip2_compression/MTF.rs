//by litissia.ben-mohand
/*
    Performs Move-To-Front (MTF) encoding on the input data.
    This transformation replaces each byte with its index 
    in a dynamically updated list of symbols.
*/ 

pub fn mtf_encode(data: &[u8]) -> Vec<u8> 
{
    let mut symbols : Vec<u8> = (0..=255).collect();
    let mut encoded = Vec::with_capacity(data.len());

    for &byte in data
    {
        let mut indx = 0;
        while symbols[indx] != byte
        {
            indx += 1;
        }

        encoded.push(indx as u8);
        let byte = symbols.remove(indx);
        symbols.insert(0,byte);
    }

    encoded
}

/*
 * Decodes Move-To-Front (MTF) encoded data back to its original form.
 */
pub fn mtf_decode(encoded: &[u8]) -> Vec<u8>
{
    let mut symbols : Vec<u8> = (0..=255).collect();
    let mut decoded = Vec::with_capacity(encoded.len());

    for &indx in encoded
    {
        let byte = symbols[indx as usize];
        
        decoded.push(byte);

        symbols.remove(indx as usize);
        symbols.insert(0,byte);
        
    }

    decoded
}
