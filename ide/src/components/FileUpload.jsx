import React, { useEffect, useState } from 'react';

import ".././styles/FileUpload.css";
import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import ContractTemplateWizard from './ContractTemplateWizard';

function FileUpload({ handleFileChange, handleUpload, handleUploadFile }) {
  return (
    <div>
      <div className='file-upload-container'>
        <ContractTemplateWizard
          handleFileChange={handleFileChange}
          handleUpload={handleUpload}
        />

      </div >
      <div className='file-upload-container' style={{ display: 'flex', flexDirection: 'row' }}>
        <input style={{ fontSize: '1.25em', backgroundColor: 'white', margin: '2%', color: 'black' }} type="file" onChange={handleFileChange} />
        <button style={{ flex: 1, fontSize: '1.25em' }} onClick={handleUploadFile}>Upload</button>
      </div>
    </div>
  )
}
export default FileUpload;