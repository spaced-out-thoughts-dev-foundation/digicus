import React from 'react';

import '.././styles/InfoHeader.css';

function InfoHeader() {
  return (
    <div className='info-header' style={{ textAlign: 'right' }}>
      Made with <span role='img' aria-label='heart'>❤️</span> by <a href='https://spaced-out-thoughts-dev-foundation.github.io/' target='_blank' rel='noopener noreferrer'>Spaced Out Thoughts Dev Foundation</a>
    </div>
  );
}

export default InfoHeader;