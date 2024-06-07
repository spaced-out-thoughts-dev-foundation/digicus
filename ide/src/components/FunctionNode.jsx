import React, { memo } from 'react';

import '.././styles/FunctionNode.css';
import '.././styles/InstructionNode.css';
import EditableTitle from './EditableTitle';

export default memo(({ data }) => {
  return (
    <div className="function-node-container">
      <EditableTitle initial_title={data.functionName} />
      <div className='instruction-node-input'>[Input]:<ul>{data.functionInputs?.map(x => <li>{x.name} ({x.type_name})</li>)}</ul> </div>
    </div>
  );
});
