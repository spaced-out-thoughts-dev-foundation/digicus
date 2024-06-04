import React, { memo } from 'react';
import { Handle, Position } from 'reactflow';

import '.././styles/InstructionNode.css';

export default memo(({ data, isConnectable }) => {
  return (
    <div>
      <div className='instruction-node-input-to-instruction'>{data.instruction.inputs.join('\n')}</div>
      <div key={data.id} style={{borderRadius: '1px', border: '1px solid black', backgroundColor: data.color}}>
        <Handle type="target" position={Position.Top} isConnectable={isConnectable}/>
        <div className='instruction-node-content'>
          <h2>{data.instruction.instruction}</h2>
        </div>
        <Handle type="source" position={Position.Bottom} isConnectable={isConnectable}/>
      </div>
      <div className='instruction-node-assign'> {data.instruction.assign ? "[Assign To]: " + data.instruction.assign : '_none_'}</div>
    </div>
  );
});
