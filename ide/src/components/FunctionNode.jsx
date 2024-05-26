import React, { memo } from 'react';

import '.././styles/FunctionNode.css';
import '.././styles/InstructionNode.css';

export default memo(({ data }) => {
  return (
    <div className="function-node-container">
      <h3>{data.functionName}</h3>
      <div className='instruction-node-input'>[Input]:<ul>{data.functionInputs?.map(x => <li>{x.name} ({x.type_name})</li>)}</ul> </div>
    </div>
  );
});
