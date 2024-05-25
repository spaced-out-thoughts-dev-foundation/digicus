import React from 'react'
import Box from '@mui/material/Box';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { dracula } from 'react-syntax-highlighter/dist/esm/styles/prism';

import "../App.css";

const codeContainerBody = (originalText, filename) => {
  return (
      filename && originalText ? 
        <SyntaxHighlighter language="rust" style={dracula}>
          {originalText}
        </SyntaxHighlighter> : ''

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