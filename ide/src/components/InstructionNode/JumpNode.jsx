import React, { memo } from 'react';
import '../../styles/InstructionNode.css';
import EditableTitle from '../EditableTitle';
import BaseInstructionNode from './BaseInstructionNode';

function JumpNodeComponent({ data }) {
  return (
    <div style={{ display: 'Flex', justifyContent: 'center' }}>
      <div style={{ flex: 1, overflow: 'auto', display: 'flex', flexDirection: 'column', padding: '1em', justifyContent: 'center' }}>
        {
          data.instruction.inputs.slice(0, -1).map((x, input_index) => <div style={{ margin: '0.1em' }} className='instruction-node-input-to-instruction'>{
            <EditableTitle initial_title={x} handleChangeTitle={(oldTitle, new_title) => data.onUpdateInputName(oldTitle, new_title, input_index)} />}<br /></div>)
        }
      </div>
    </div>
  );
}

export default memo(({ data, isConnectable }) => {
  return (
    <BaseInstructionNode data={data} isConnectable={isConnectable} specificNodeComponent={JumpNodeComponent} />
  );
});
