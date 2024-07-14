import React, { memo } from 'react';

import '../../styles/FunctionNode.css';
import '../../styles/InstructionNode.css';

export default memo(({ data }) => {
  return (
    <div className="function-node-container" style={{ backgroundColor: data.color, height: '100%', width: '100%' }}>
      {/* <h3>Scope: {data.scope}</h3> */}
    </div>
  );
});
