import React, { memo } from 'react';
import { Handle, Position } from 'reactflow';
import Paper from '@mui/material/Paper';
import '.././styles/InstructionNode.css';

export default memo(({ data, isConnectable }) => {
  return (
    <div>
      <div key={data.id}>
        <Handle type="target" position={Position.Top} isConnectable={isConnectable} />
        <div className='instruction-node-content'>
          <Paper elevation={3} style={{ backgroundColor: data.color, borderRadius: '1px', border: '1px solid black' }}>
            <div style={{ display: 'Flex', justifyContent: 'center' }}>
              <h2 style={{ flex: 1, justifyContent: 'center', display: 'Flex', alignContent: 'center', alignItems: 'center' }}>{data.instruction.instruction}</h2>
              <div style={{ flex: 1, overflow: 'auto', display: 'flex', flexDirection: 'column', padding: '1em', justifyContent: 'center' }}>
                {data.instruction.inputs.map(x => <div style={{ margin: '0.1em' }} className='instruction-node-input-to-instruction'>{x}<br /></div>)}
              </div>
            </div>
          </Paper>
        </div>
        <Handle type="source" position={Position.Bottom} isConnectable={isConnectable} />
      </div>
      {data.instruction.assign ? <div className='instruction-node-assign'>{"[Assign To]: " + data.instruction.assign}</div> : null}
    </div>
  );
});
