import React, { memo } from 'react';
import { Handle, Position } from 'reactflow';

import '../App.css';

export default memo(({ data, isConnectable }) => {
  return (
    <div key={data.id} style={{borderRadius: '1px', border: '1px solid black', backgroundColor: data.color}}>
      <div className='instruction-node-input'>[Input:]<ul>{data.instruction.inputs.map(x => <li>{x}</li>)}</ul> </div>
      <Handle type="target" position={Position.Top} isConnectable={isConnectable}/>
      <div className='instruction-node-content'>
        <h2>{data.instruction.instruction}</h2>
      </div>
      <Handle type="source" position={Position.Bottom} isConnectable={isConnectable}/>
      <div className='instruction-node-assign'>[Assign To]: {data.instruction.assign ? data.instruction.assign : '_none_'}</div>
    </div>
  );
});
