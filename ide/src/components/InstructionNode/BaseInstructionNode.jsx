import React, { memo } from 'react';
import { Handle, Position } from 'reactflow';
import Paper from '@mui/material/Paper';
import '../../styles/InstructionNode.css';
import { Tooltip } from '@mui/material';

export default memo(({ data, isConnectable, specificNodeComponent }) => {
  return (
    <div style={{ height: `105%`, padding: '2px', width: '100%', border: '2px black dashed', marginTop: '11px', marginBottom: '11px' }}>
      <div key={data.id}>
        <div className='instruction-node-content' style={{ width: '100%' }}>
          <Tooltip title={data.description}>
            <h4 style={{
              flex: 1,
              justifyContent: 'left',
              display: 'Flex',
              alignContent: 'left',
              alignItems: 'left',
              padding: '0',
              margin: '0',
            }}>{data.instruction.instruction}</h4>

            <Paper elevation={3} style={{ height: '100%', backgroundColor: data.color, borderRadius: '1px', border: '1px solid black', paddingTop: '0', marginTop: '0', width: '100%', paddingBottom: '2px' }}>
              {data.isTop ? null : (data.displayHandle ? <Handle type="target" position={Position.Top} isConnectable={isConnectable} /> : null)}

              {specificNodeComponent({ data, isConnectable })}
              {data.isBottom ? null : (data.displayHandle ? <Handle type="source" position={Position.Bottom} isConnectable={isConnectable} /> : null)}
              {data.instruction.assign ? <div className='instruction-node-assign'>{"[Assign To]: " + data.instruction.assign}</div> : null}
            </Paper>
          </Tooltip>
        </div>
      </div>
    </div >
  );
});
