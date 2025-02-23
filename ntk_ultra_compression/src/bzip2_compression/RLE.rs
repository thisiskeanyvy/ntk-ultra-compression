//by litissia.ben-mohand

/*
    Performs Run-Length Encoding on the input data : 
    it replaces consecutive repeated values with a count 
    followed by the value itself.
*/

pub fn rle_encode(data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut count = 1;
    let mut last_byte = data[0];

    for i in 1..data.len()
    {
        if data[i] == last_byte
        {
            if count == 255
            {
                //store the maximum value possible for 
                //count (255) and reset counter
                encoded.push(255);
                encoded.push(last_byte);
                count = 1; 
            } 
            else
            {
                count += 1;
            }
        }
        else
        {
            encoded.push(count);
            encoded.push(last_byte);
            count = 1;
            last_byte = data[i];
        }
    }

    encoded.push(count);
    encoded.push(last_byte as u8);
    encoded
}


/*
    Decodes a Run-Length Encoded input back to its original form
*/

pub fn rle_decode(encoded: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();
    
    for i in (0..encoded.len()).step_by(2)
    {
        for j in 0..encoded[i]
        {
            //encoded[i] is the number of repetitions
            //encoded[i+1] is the value repeated
            decoded.push(encoded[i+1]);
        }
    }

    decoded
}
