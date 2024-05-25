import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow, { MarkerType } from 'reactflow';
import InstructionNode from './InstructionNode';
import CodeContainer from './CodeContainer';

import "../App.css";

const nodeTypes = {
  instructionNode: InstructionNode,
};

function determineInstructionColor(instructionName, supportedInstructions, supportedInstructionToColor) {
  return supportedInstructionToColor(tryGetSupportedInstruction(instructionName, supportedInstructions)[0]);
};

function tryGetSupportedInstruction(instructionName, supportedInstructions) {
  return supportedInstructions.filter(x => x.name.toUpperCase() === instructionName.toUpperCase());
};

function constructNode(instruction, index, function_number, instructionColor) {
  return { 
    id: `${index}|${function_number}`, 
    data: { 
      color: instructionColor,
      id: `${index}|${function_number}`,
      instruction: instruction, 
      label: `${instruction.instruction.toUpperCase()} ${instruction.inputs ? `(${instruction.inputs.join(',')})` : ''}` 
    }, 
    position: { x: 200 * (function_number), y: 150 * (index + 1) },
    type: 'instructionNode', 
  };
};

function nodes(function_data, supportedInstructions, supportedInstructionToColor, function_number) {
  let function_json_data = JSON.parse(function_data);
  let all_function_nodes = function_json_data.instructions
    .filter((instruction) => !!tryGetSupportedInstruction(instruction.instruction, supportedInstructions))
    .map((instruction, index) => {
      return constructNode(
        instruction, 
        index+1, 
        function_number,
        determineInstructionColor(instruction.instruction, supportedInstructions, supportedInstructionToColor)
      );
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
    }};
};

function edges(function_data, function_number) {
  return JSON.parse(function_data)
    .instructions
    .slice(1)
    .map((_, index) => constructEdge(index, function_number));
};

function ContractContainer({functions, supportedInstructions, supportedInstructionToColor, originalText, filename}) {
    return (
      <div className='contract-container-container'>
        <Box className='contract-container-box'>
          {
            functions ?
              <ReactFlow 
                nodes={functions.map((f, i) => nodes(f, supportedInstructions, supportedInstructionToColor, i)).flatMap(x => x)} 
                edges={functions.map((f, i) => edges(f, i))}
                fitView={{padding: 10}}
                panOnDrag={false}
                zoomOnScroll={false}
                zoomOnPinch={false}
                zoomOnDoubleClick={false}
                nodeTypes={nodeTypes}
              />
               : ''
            }
        </Box>
       <CodeContainer originalText={originalText} filename={filename} />
      </div>
    );
}

export default ContractContainer;