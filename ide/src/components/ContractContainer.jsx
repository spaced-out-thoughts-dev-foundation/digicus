import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow, { Controls, MarkerType, MiniMap } from 'reactflow';
import InstructionNode from './InstructionNode';
import FunctionNode from './FunctionNode';
import CodeContainer from './CodeContainer';
// import UserDefinedTypesContainer from './UserDefinedTypesContainer';

import ".././styles/ContractContainer.css";

const nodeTypes = {
  instructionNode: InstructionNode,
  functionNode: FunctionNode,
};

function determineInstructionColor(instructionName, supportedInstructions, supportedInstructionToColor) {
  return supportedInstructionToColor(tryGetSupportedInstruction(instructionName, supportedInstructions)[0]);
};

function tryGetSupportedInstruction(instructionName, supportedInstructions) {
  return supportedInstructions.filter(x => x.name.toUpperCase() === instructionName.toUpperCase());
};

function constructNode(instruction, index, function_number, instructionColor, onUpdateInputName) {
  return {
    id: `${index}|${function_number}`,
    data: {
      color: instructionColor,
      id: `${index}|${function_number}`,
      instruction: instruction,
      label: `${instruction.instruction.toUpperCase()} ${instruction.inputs ? `(${instruction.inputs.join(',')})` : ''}`,
      onUpdateInputName: (oldTitle, newTitle, instruction_index) => onUpdateInputName(oldTitle, newTitle, instruction, instruction_index, function_number, index)
    },
    position: { x: 0 * (function_number), y: index === 1 ? 150 : 50 + 200 * (index) },
    type: 'instructionNode',
    parentId: `f-${function_number}`
  };
};

function nodes(function_data, supportedInstructions, supportedInstructionToColor, function_number, onUpdateFunctionName, onUpdateInputName) {
  let function_json_data = JSON.parse(function_data);
  console.log("function_json_data", function_json_data)
  let all_function_nodes = function_json_data.instructions
    .map((instruction) => JSON.parse(instruction))
    .filter((instruction) => !!tryGetSupportedInstruction(instruction.instruction, supportedInstructions))
    .map((instruction, index) => {
      return constructNode(
        instruction,
        index + 1,
        function_number,
        determineInstructionColor(instruction.instruction, supportedInstructions, supportedInstructionToColor),
        onUpdateInputName
      );
    })

  all_function_nodes.unshift({
    id: `f-${function_number}`,
    data: {
      functionName: function_json_data.name,
      functionInputs: function_json_data.inputs,
      onUpdateFunctionName: (newTitle, oldTitle) => onUpdateFunctionName(newTitle, oldTitle)
    },
    position: { x: 250 * (function_number * 1.5), y: 0 },
    style: {
      color: 'black',
      fontSize: '1em',
      borderRadius: 10,
      width: 300,
      height: 200 * (function_json_data.instructions.length + 1) + 25,
      marginLeft: '-50px',
      backgroundColor: 'rgba(255, 255, 0, 0.15)',
      textShadow: '0.5px 0.5px 0.5px black',
      border: '1px solid black'
    },
    extent: 'parent',
    type: 'functionNode',
  })

  return all_function_nodes;
}


function constructEdge(index, function_number) {
  return {
    id: `e-${index + 1}|${function_number}`,
    source: `${index + 1}|${function_number}`,
    target: `${index + 2}|${function_number}`,
    markerEnd: {
      type: MarkerType.ArrowClosed,
      width: 20,
      height: 20,
    }
  };
};

function edges(function_data, function_number) {
  return JSON.parse(function_data)
    .instructions
    .slice(1)
    .map((_, index) => constructEdge(index, function_number));
};

function ContractContainer({
  functions, supportedInstructions, supportedInstructionToColor, originalText, filename,
  showCodeContainer, showUserDefinedTypes, userDefinedTypes, generatedText, onUpdateFunctionName, onUpdateInputName
}) {
  return (
    <div className='contract-container-container'>
      <Box className='contract-container-box'>
        {
          functions ?
            <ReactFlow
              nodes={functions.map((f, i) => nodes(f, supportedInstructions, supportedInstructionToColor, i, onUpdateFunctionName, onUpdateInputName)).flatMap(x => x)}
              edges={functions.map((f, i) => edges(f, i)).flatMap(x => x)}
              fitView={{ padding: 100 }}
              nodeTypes={nodeTypes}
            // panOnScroll={false}
            // zoomOnScroll={false}
            // zoomOnPinch={true}
            >
              <Controls />
              <MiniMap
                pannable={true}
                zoomable={true}
              />
            </ReactFlow>
            : ''
        }
      </Box>
      <div className='code-container-div'>
        {showCodeContainer ? <CodeContainer originalText={originalText} filename={filename} originalSource={true} /> : ''}
        {/* {showUserDefinedTypes ? <UserDefinedTypesContainer userDefinedTypes={userDefinedTypes} /> : ''} */}
        {showUserDefinedTypes ? <CodeContainer originalText={generatedText} filename={filename} originalSource={false} /> : ''}
      </div>
    </div>
  );
}

export default ContractContainer;