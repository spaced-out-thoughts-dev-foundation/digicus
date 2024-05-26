import React from 'react'

import ".././styles/FileUpload.css";

function FileUpload({handleFileChange, handleUpload}) {
  return (
    <div className='file-upload-container'>
      <input style={{fontSize: '1.25em', backgroundColor: 'white', margin: '2%', color: 'black'}} type="file" onChange={handleFileChange} />
      <button style={{flex: 1, fontSize: '1.25em'}}onClick={handleUpload}>Upload</button>
    </div>
  )
}
export default FileUpload;