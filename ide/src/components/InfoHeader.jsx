import React from 'react';

import '.././styles/InfoHeader.css';

function handleDeploy() {
  console.log('Deploying...');
}

function InfoHeader() {
  return (
    <div className='info-header' style={{ textAlign: 'right' }}>
      <button style={{ width: '20%', backgroundColor: '#27cfe6', border: 'none', padding: '10px', color: 'white', cursor: 'pointer', borderRadius: '5px', marginBottom: '10px' }}>
        <a href='https://spaced-out-thoughts-development-foundation.instatus.com/' target='_blank' rel='noopener noreferrer' style={{ textDecoration: 'none', color: 'inherit', fontSize: '0.75vw' }}>
          Service Status
        </a>
      </button>
      <br></br>
      Made with <span role='img' aria-label='heart'>❤️</span> by <a href='https://spaced-out-thoughts-dev-foundation.github.org/' target='_blank' rel='noopener noreferrer' style={{ textDecoration: 'none', color: 'inherit', fontSize: '0.75vw' }}>Spaced Out Thoughts Dev Foundation</a>
    </div>
  );
}

export default InfoHeader;