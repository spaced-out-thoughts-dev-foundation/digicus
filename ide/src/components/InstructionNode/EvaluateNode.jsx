import React, { memo } from 'react';
import '../../styles/InstructionNode.css';
import EditableTitle from '../EditableTitle';
import BaseInstructionNode from './BaseInstructionNode';

function EvaluateNodeComponent({ data }) {
  const splittedInstructionZero = data.instruction.inputs[0].split('.');
  let instructions = data.instruction.inputs.slice(1);
  let methodName = data.instruction.inputs[0];
  let isCallOnThing = splittedInstructionZero.length > 1;
  if (isCallOnThing) {
    methodName = splittedInstructionZero[1];
    instructions.unshift(splittedInstructionZero[0]);
  }

  return (
    <div>
      <h4 style={{ flex: 1, justifyContent: 'center', display: 'Flex', alignContent: 'center', alignItems: 'center', textAlign: 'center', overflow: 'scroll' }}>
        <div style={{ fontSize: '1.5em', justifyContent: 'center', alignItems: 'center', textAlign: 'center' }}>[{methodName}]</div>
      </h4>

      <div style={{ display: 'Flex', justifyContent: 'center', flex: 4 }}>
        <div style={{ flex: 4, overflow: 'auto', display: 'flex', flexDirection: 'column', padding: '1em', justifyContent: 'center' }}>
          {
            instructions.map((x, input_index) => <div style={{ margin: '0.1em' }} className='instruction-node-input-to-instruction'>{
              <EditableTitle initial_title={x} isCallOnThing={isCallOnThing && input_index === 0} handleChangeTitle={(oldTitle, new_title) => data.onUpdateInputName(oldTitle, new_title, input_index)} />}<br /></div>)
          }
        </div>
      </div>
    </div>
  );
}

export default memo(({ data, isConnectable }) => {
  return (
    <BaseInstructionNode data={data} isConnectable={isConnectable} specificNodeComponent={EvaluateNodeComponent} />
  );
});
