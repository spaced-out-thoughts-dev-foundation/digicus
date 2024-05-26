import React, { useEffect, useState } from 'react';

import '.././styles/InfoHeader.css';

function InfoHeader() {
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch('https://block-render-engine.vercel.app/api/ide_version')
      .then(response => {
        return response.json()
      })
      .then(json => setData(json))
      .catch(error => console.error(error));
  }, []);

  return (
    <div className='info-header'>
      Relevant Software Dependencies:
        {data ? <p>[dtr_core]: {data.dtr_core}</p> : <p>Loading...</p>}
        {data ? <p>[block render engine]: {data.block_render_engine}</p> : <p>Loading...</p>}
    </div>
  );
}

export default InfoHeader;