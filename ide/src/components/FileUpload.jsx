import React, { useEffect, useState } from 'react';

import ".././styles/FileUpload.css";
import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';

function FileUpload({ handleFileChange, handleUpload }) {
  const [contract, setContract] = useState("hello_world");

  const handleChange = (event) => {
    setContract(event.target.value);
  }

  console.log(contract);
  return (
    <div className='file-upload-container'>
      {/* <input style={{fontSize: '1.25em', backgroundColor: 'white', margin: '2%', color: 'black'}} type="file" onChange={handleFileChange} /> */}
      <FormControl fullWidth style={{
        flex: 1,
      }}>
        <InputLabel id="demo-simple-select-label" style={{ fontSize: "1.25em" }}>Example Contract</InputLabel>
        <Select
          labelId="demo-simple-select-label"
          id="demo-simple-select"
          value={contract}
          label="Template Contract"
          onChange={handleChange}
          style={{ fontSize: '1m', backgroundColor: 'white', margin: '2%', color: 'black' }}
        >
          <MenuItem value={"hello_world"}>Hello World</MenuItem>
          <MenuItem value={"increment"}>Increment</MenuItem>
          {/* <MenuItem value={"custom_types"}>Custom Types</MenuItem> */}
          <MenuItem value={"logging"}>Logging</MenuItem>
          {/* <MenuItem value={"errors"}>Errors</MenuItem> */}
          {/* <MenuItem value={"events"}>Events</MenuItem> */}

        </Select>
      </FormControl>
      <button style={{ flex: 1, fontSize: '1.25em' }} onClick={() => handleUpload(contract)}>Upload</button>

    </div >
  )
}
export default FileUpload;