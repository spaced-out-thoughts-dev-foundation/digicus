import React, { memo } from 'react';
import { Handle, Position } from 'reactflow';

export default memo(({ data, isConnectable }) => {
  return (
    <div style={{borderRadius: '1px', border: '1px solid black', backgroundColor: data.color}}>
     <div style={{
      backgroundColor: 'white', 
      margin: '2px', 
      padding: '2px',
      textAlign: 'left',
      fontSize: '5pt',
    }}>[Input:]<ul>{data.instruction.inputs.map(x => <li>{x}</li>)}</ul> </div>
      <Handle
        type="target"
        position={Position.Top}
        style={{ background: '#555' }}
        isConnectable={isConnectable}
      />
      <div style={{ 
        padding: 10, 
        fontSize: '0.5em',
        textShadow: '1px 1px 1px gray',
      }}>
        <h2>{data.instruction.instruction}</h2>
      </div>
      <Handle
        type="source"
        position={Position.Bottom}
        style={{ background: '#555' }}
        isConnectable={isConnectable}
      />
      <div style={{
        backgroundColor: 'white', 
        margin: '2px', 
        padding: '2px',
        textAlign: 'left',
        fontSize: '5pt',
      }}>[Assign To]: {data.instruction.assign ? data.instruction.assign : '_none_'}</div>
      </div>
  );
});
