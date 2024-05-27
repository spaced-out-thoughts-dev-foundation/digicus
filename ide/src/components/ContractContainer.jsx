import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow, { Controls, MarkerType, MiniMap } from 'reactflow';
import InstructionNode from './InstructionNode';
import FunctionNode from './FunctionNode';
import CodeContainer from './CodeContainer';
import UserDefinedTypesContainer from './UserDefinedTypesContainer';

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

function constructNode(instruction, index, function_number, instructionColor) {
  return { 
    id: `${index}|${function_number}`, 
    data: { 
      color: instructionColor,
      id: `${index}|${function_number}`,
      instruction: instruction, 
      label: `${instruction.instruction.toUpperCase()} ${instruction.inputs ? `(${instruction.inputs.join(',')})` : ''}` 
    }, 
    position: { x: 0 * (function_number), y: 150 * (index) },
    type: 'instructionNode',
    parentId: `f-${function_number}`
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

    all_function_nodes.unshift({
      id: `f-${function_number}`, 
      data: { 
        functionName: function_json_data.name,
        functionInputs: function_json_data.inputs,
      }, 
      position: { x: 250 * (function_number * 1.5), y: 0 },
      style: {
        color: 'black',
        fontSize: '1em',
        borderRadius: 10,
        width: 300,
        height: 150 * (function_json_data.instructions.length + 1),
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
    }};
};

function edges(function_data, function_number) {
  return JSON.parse(function_data)
    .instructions
    .slice(1)
    .map((_, index) => constructEdge(index, function_number));
};

function ContractContainer({functions, supportedInstructions, supportedInstructionToColor, originalText, filename, showCodeContainer, showUserDefinedTypes, userDefinedTypes}) {
    return (
      <div className='contract-container-container'>
        <Box className='contract-container-box'>
          {
            functions ?
              <ReactFlow 
                nodes={functions.map((f, i) => nodes(f, supportedInstructions, supportedInstructionToColor, i)).flatMap(x => x)} 
                edges={functions.map((f, i) => edges(f, i)).flatMap(x => x)}
                fitView={{padding: 100}}
                nodeTypes={nodeTypes}
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
          { showCodeContainer ?  <CodeContainer originalText={originalText} filename={filename} /> : '' }
          { showUserDefinedTypes ?  <UserDefinedTypesContainer userDefinedTypes={userDefinedTypes} /> : '' }
        </div>
      </div>
    );
}

export default ContractContainer;