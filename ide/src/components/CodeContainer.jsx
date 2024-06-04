import React from 'react'
import Box from '@mui/material/Box';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { dracula } from 'react-syntax-highlighter/dist/esm/styles/prism';

import ".././styles/CodeContainer.css";

const codeContainerBody = (originalText, filename) => {
  return (
      filename && originalText ? 
      <div style={{fontSize: '0.75em', overflow: 'auto'}}>
        <SyntaxHighlighter language="rust" style={{...dracula, overflow: 'auto'}}>
          {originalText}
        </SyntaxHighlighter> </div> : ''

  );
}

function CodeContainer({originalText, filename}) {
  return (
    <Box className="code-container">
      <h2>Source Code</h2>
      {codeContainerBody(originalText, filename)}
    </Box>
  )
}
export default CodeContainer;