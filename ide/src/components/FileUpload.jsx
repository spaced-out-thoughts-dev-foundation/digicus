import React from 'react'

import ".././styles/FileUpload.css";
import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';

function FileUpload({ handleFileChange, handleUpload }) {
  let contract = "hello_world";
  const handleChange = (event) => {
    contract = event.target.value;
    handleFileChange(event);
  }
  return (
    <div className='file-upload-container'>
      {/* <input style={{fontSize: '1.25em', backgroundColor: 'white', margin: '2%', color: 'black'}} type="file" onChange={handleFileChange} />
      <button style={{flex: 1, fontSize: '1.25em'}}onClick={handleUpload}>Upload</button> */}
      <FormControl fullWidth>
        <InputLabel id="demo-simple-select-label">Age</InputLabel>
        <Select
          labelId="demo-simple-select-label"
          id="demo-simple-select"
          value={contract}
          label="Template Contract"
          onChange={handleChange}
          style={{ fontSize: '1.25em', backgroundColor: 'white', margin: '2%', color: 'black' }}
        >
          <MenuItem value={"hello_world"}>Hello World</MenuItem>
          <MenuItem value={"increment"}>Increment</MenuItem>
        </Select>
      </FormControl>
    </div >
  )
}
export default FileUpload;