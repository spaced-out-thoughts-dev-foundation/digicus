import React, { memo } from 'react';
import '../../styles/InstructionNode.css';
import EditableTitle from '../EditableTitle';
import BaseInstructionNode from './BaseInstructionNode';

function GotoNodeComponent({ data }) {
  return (
    <div style={{ display: 'Flex', justifyContent: 'center' }}>
    </div>
  );
}

export default memo(({ data, isConnectable }) => {
  return (
    <BaseInstructionNode data={data} isConnectable={isConnectable} specificNodeComponent={GotoNodeComponent} />
  );
});
