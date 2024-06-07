import React, { memo } from 'react';
import { Handle, Position } from 'reactflow';
import Paper from '@mui/material/Paper';
import '.././styles/InstructionNode.css';

export default memo(({ data, isConnectable }) => {
  return (
    <div>
      <div className='instruction-node-input-to-instruction'>{data.instruction.inputs.map(x => <div>{"-> " + x}<br /></div>)}</div>
      <div key={data.id}>
        <Handle type="target" position={Position.Top} isConnectable={isConnectable} />
        <div className='instruction-node-content'>
          <Paper elevation={3} style={{ backgroundColor: data.color, borderRadius: '1px', border: '1px solid black' }}>
            <h2>{data.instruction.instruction}</h2>
          </Paper>
        </div>
        <Handle type="source" position={Position.Bottom} isConnectable={isConnectable} />
      </div>
      <div className='instruction-node-assign'> {data.instruction.assign ? "[Assign To]: " + data.instruction.assign : '_none_'}</div>
    </div>
  );
});
