import React, { memo } from 'react';
import '../../styles/InstructionNode.css';
import EditableTitle from '../EditableTitle';
import BaseInstructionNode from './BaseInstructionNode';

function binaryInstructionNameToOperator(instructionName) {
  if (instructionName === 'add') return '+';
  if (instructionName === 'subtract') return '-';
  if (instructionName === 'multiply') return '*';
  if (instructionName === 'divide') return '/';
  if (instructionName === 'and') return '&&';
  if (instructionName === 'or') return '||';
}

function BinaryNodeComponent({ data }) {
  let lhs = data.instruction.inputs[0];
  let rhs = data.instruction.inputs[1];

  return (
    <div style={{ display: 'flex', justifyContent: 'center', flexDirection: 'row' }}>


      <div style={{
        flex: 1, overflow: 'auto', display: 'flex', flexDirection: 'row', padding: '1em', justifyContent: 'center'
      }}>
        <div style={{
          alignItems: 'center',
          alignContent: 'center',
          justifyContent: 'center',
          margin: '0',
          padding: '0',
          backgroundColor: 'rgba(255, 255, 255, 0.15)',
          textAlign: 'center',
          fontSize: '0.5em',
          height: '100%',
          border: 'black 1px dotted',
          borderRadius: '10px',
          overflowX: 'auto',
          width: '100%',
          display: 'flex',
        }}>
          <EditableTitle initial_title={lhs} handleChangeTitle={(oldTitle, new_title) => data.onUpdateInputName(oldTitle, new_title, 0)} />
          <br />
        </div>
        <h2 style={{ flex: 1, justifyContent: 'center', display: 'Flex', alignContent: 'center', alignItems: 'center', padding: '0.5em' }}>
          {binaryInstructionNameToOperator(data.instruction.instruction)}
        </h2>
        <div style={{
          alignItems: 'center',
          alignContent: 'center',
          justifyContent: 'center',
          margin: '0',
          padding: '0',
          backgroundColor: 'rgba(255, 255, 255, 0.15)',
          textAlign: 'center',
          fontSize: '0.7em',
          height: '100%',
          border: 'black 1px dotted',
          borderRadius: '10px',
          overflowX: 'auto',
          width: '100%',
          display: 'flex',
        }}>
          <EditableTitle initial_title={rhs} handleChangeTitle={(oldTitle, new_title) => data.onUpdateInputName(oldTitle, new_title, 1)} />
          <br />
        </div>
      </div>
    </div>
  );
}

export default memo(({ data, isConnectable }) => {

  return (
    <BaseInstructionNode data={data} isConnectable={isConnectable} specificNodeComponent={BinaryNodeComponent} />
  );
});
