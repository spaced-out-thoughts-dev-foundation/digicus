import React from 'react'
import Box from '@mui/material/Box';
import ReactFlow, { Controls, MarkerType, MiniMap } from 'reactflow';
import InstructionNode from './InstructionNode/UnknownInstructionNode';
import FunctionNode from './InstructionNode/FunctionNode';
import ScopeNode from './InstructionNode/ScopeNode';
import CodeContainer from './CodeContainer';
// import UserDefinedTypesContainer from './UserDefinedTypesContainer';

import UnknownInstructionNode from './InstructionNode/UnknownInstructionNode';
import AssignNode from './InstructionNode/AssignNode';
import BinaryNode from './InstructionNode/BinaryNode';
import EndOfIterationNode from './InstructionNode/EndOfIterationCheckNode';
import EvaluateNode from './InstructionNode/EvaluateNode';
import ExitWithMessageNode from './InstructionNode/ExitWithMessageNode';
import GotoNode from './InstructionNode/GotoNode';
import IncrementNode from './InstructionNode/IncrementNode';
import InstantiateObjectNode from './InstructionNode/InstantiateObjectNode';
import JumpNode from './InstructionNode/JumpNode';
import PrintNode from './InstructionNode/PrintNode';
import ReturnNode from './InstructionNode/ReturnNode';
import TryAssignNode from './InstructionNode/TryAssignNode';
import UnaryNode from './InstructionNode/UnaryNode';

import _ from 'lodash';

import ".././styles/ContractContainer.css";
import { determineInstructionHeight, determineInstructionNodeType } from '../common/InstructionNode';
import EndOfIterationCheckNode from './InstructionNode/EndOfIterationCheckNode';
import { color } from 'd3';

const nodeTypes = {
  functionNode: FunctionNode,
  unknownInstructionNode: UnknownInstructionNode,
  assignNode: AssignNode,
  binaryNode: BinaryNode,
  endOfIterationNode: EndOfIterationNode,
  evaluateNode: EvaluateNode,
  exitWithMessageNode: ExitWithMessageNode,
  gotoNode: GotoNode,
  incrementNode: IncrementNode,
  instantiateObjectNode: InstantiateObjectNode,
  jumpNode: JumpNode,
  printNode: PrintNode,
  returnNode: ReturnNode,
  tryAssignNode: TryAssignNode,
  unaryNode: UnaryNode,
  scopeNode: ScopeNode,
  endOfIterationCheckNode: EndOfIterationCheckNode
};

function determineInstructionInfo(instructionName, supportedInstructions, supportedInstructionToColor) {
  let supportedInstruction = tryGetSupportedInstruction(instructionName, supportedInstructions)[0];

  if (!supportedInstruction) {
    return {
      color: 'white',
      description: 'Unsupported instruction'
    };
  }

  return {
    color: supportedInstructionToColor(supportedInstruction),
    description: supportedInstruction.description
  };
}

function tryGetSupportedInstruction(instructionName, supportedInstructions) {
  return supportedInstructions.filter(x => x.name.toUpperCase() === instructionName.toUpperCase());
};

const NODE_WIDTH = 450;

function constructNode(instruction, index, function_number, supportedInstructionInfo, onUpdateInputName, numInstructions, positionY, height, positionX) {
  const modded_instruction = _.cloneDeep(instruction);
  modded_instruction.inputs = modded_instruction.inputs.filter((x) => x !== '&');
  return {
    id: `${index}|${function_number}`,
    data: {
      isTop: index === 1,
      height: height,
      isBottom: index === numInstructions || instruction.instruction === 'exit_with_message' || instruction.instruction === 'return',
      // TODO: fix this
      displayHandle: true,
      color: supportedInstructionInfo['color'],
      description: supportedInstructionInfo['description'],
      id: `${index}|${function_number}`,
      instruction: instruction,
      label: `${instruction.instruction.toUpperCase()} ${instruction.inputs ? `(${instruction.inputs.join(',')})` : ''}`,
      onUpdateInputName: (oldTitle, newTitle, instruction_index) => onUpdateInputName(oldTitle, newTitle, instruction, instruction_index, function_number, index)
    },
    position: { x: positionX + (function_number), y: positionY },
    style: {
      width: NODE_WIDTH,
    },
    type: determineInstructionNodeType(instruction.instruction),
    // parentId: `s-${instruction.scope}`
  };
};

function nodes(function_data, supportedInstructions, supportedInstructionToColor, function_number, onUpdateFunctionName, onUpdateInputName, positionXForFunction) {
  let function_json_data = JSON.parse(function_data);
  let height = 75 + (15 * function_json_data.inputs.length);
  let scopeMap = {
    0: 0
  }
  let scopes = 1;
  let scopeWidth = NODE_WIDTH + 100;
  let all_function_nodes = function_json_data.instructions
    .map((instruction) => JSON.parse(instruction))
    .filter((instruction) => !!tryGetSupportedInstruction(instruction.instruction, supportedInstructions))
    .map((instruction, index) => {
      // don't show gotos, however, we need in order to maintain index for edge calculation
      if (instruction.instruction === 'goto') {
        return {
          node: null, instruction: instruction.instruction
        }
      }
      if (instruction.inputs == null) {
        instruction.inputs = [];
      }
      let instructionHeight = determineInstructionHeight(determineInstructionNodeType(instruction.instruction), instruction.inputs.length, instruction.assign != null, instruction.instruction, instruction.inputs);
      let currentHeight = height + 100;
      height += instructionHeight + 100;

      if (instruction.scope != null) {
        if (scopeMap[instruction.scope] == null) {
          scopeMap[instruction.scope] = scopes * scopeWidth;
          scopes += 1;
        }
      }

      return {
        node: constructNode(
          instruction,
          index + 1,
          function_number,
          determineInstructionInfo(instruction.instruction, supportedInstructions, supportedInstructionToColor),
          onUpdateInputName,
          function_json_data.instructions.length,
          currentHeight,
          instructionHeight,
          scopeMap[instruction.scope] + positionXForFunction
        ), instruction: instruction.instruction
      }
    })
    .filter((x) => x.node != null)
    .map((x) => x.node);

  let indexForScope = 0;
  for (const [key, value] of Object.entries(scopeMap)) {
    let color = [
      'green',
      'blue',
      'red',
      'yellow',
      'purple',
      'orange',
      'pink',
      'brown',
      'gray',
      'black'
    ][parseInt(indexForScope) % 10
    ];
    all_function_nodes.unshift({
      id: `s-${key}`,
      data: {
        scope: key,
        color: `rgba(255, 255, ${255} 0)`
      },
      position: { x: value + 5, y: 75 + (15 * function_json_data.inputs.length) - 5 },
      style: {
        // color: 'black',
        // fontSize: '1em',
        borderRadius: 10,
        width: scopeWidth - 5 - 5,
        height: height + 50 - (75 + (15 * function_json_data.inputs.length)) - 5,
        marginLeft: '-50px',
        backgroundColor: color,
        // backgroundColor: `rgba(255, 255, 0, 0)`,
        textShadow: '0.5px 0.5px 0.5px black',
        border: '1px solid black',
        opacity: 0.05
      },
      extent: 'parent',
      type: 'scopeNode',
      parentId: `f-${function_number}`
    })

    indexForScope += 1;
  };



  all_function_nodes.unshift({
    id: `f-${function_number}`,
    data: {
      functionName: function_json_data.name,
      functionInputs: function_json_data.inputs,
      onUpdateFunctionName: (newTitle, oldTitle) => onUpdateFunctionName(newTitle, oldTitle)
    },
    position: { x: positionXForFunction, y: 0 },
    style: {
      color: 'black',
      fontSize: '1em',
      borderRadius: 10,
      width: scopeWidth * scopes,
      height: height + 50,
      marginLeft: '-50px',
      backgroundColor: `rgba(255, 255, 0, 0.15)`,
      textShadow: '0.5px 0.5px 0.5px black',
      border: '1px solid black'
    },
    extent: 'parent',
    type: 'functionNode',
  })

  return { all_function_nodes, last_width: scopeWidth * scopes };
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
  let scopes = {};
  let ids = {};
  let indexs = {};

  let edges = [];

  JSON.parse(function_data)
    .instructions
    .forEach((x, index) => {
      let json_x = JSON.parse(x);
      if (json_x.scope != null) {
        if (scopes[json_x.scope] == null) {
          scopes[json_x.scope] = [];
        }
        scopes[json_x.scope].push({ instruction: json_x, index: index });
      }

      ids[json_x.id] = { instruction: json_x, index: index };
      indexs[index] = { instruction: json_x, index: index };
    });

  JSON.parse(function_data)
    .instructions
    .forEach((x, index) => {
      let json_x = JSON.parse(x);
      if (json_x.instruction === 'jump') {
        if (scopes[json_x.scope]) {
          let scope_of_interest = -1;
          let scope_of_interest_2 = -1;
          if (json_x.inputs.length == 1) {
            scope_of_interest = scopes[json_x.inputs[0]];
          } else if (json_x.inputs.length == 2) {
            scope_of_interest = scopes[json_x.inputs[1]];
            scope_of_interest_2 = scopes[json_x.scope];
          }

          if (scope_of_interest != -1) {
            let point_to = null;
            for (let i = 0; i < scope_of_interest.length; i++) {
              if (scope_of_interest[i].index > index) {
                point_to = scope_of_interest[i];
                break;
              }
            }

            if (point_to != null) {
              edges.push({
                id: `e-${index + 1}|${function_number}-true`,
                source: `${index + 1}|${function_number}`,
                target: `${point_to.index + 1}|${function_number}`,
                markerEnd: {
                  type: MarkerType.ArrowClosed,
                  width: 20,
                  height: 20,
                  color: json_x.inputs.length == 1 ? 'gray' : 'green'
                },
                label: json_x.inputs.length == 1 ? 'unconditional jump' : 'true',
                style: {
                  strokeWidth: '2px',
                  stroke: json_x.inputs.length == 1 ? 'gray' : 'green'
                }
              });
            }
          }

          if (scope_of_interest_2 != -1) {
            let point_to = null;
            for (let i = 0; i < scope_of_interest_2.length; i++) {
              if (scope_of_interest_2[i].index > index) {
                point_to = scope_of_interest_2[i];
                break;
              }
            }
            if (point_to != null) {
              edges.push({
                id: `e-${index + 1}|${function_number}-false`,
                source: `${index + 1}|${function_number}`,
                target: `${point_to.index + 1}|${function_number}`,
                markerEnd: {
                  type: MarkerType.ArrowClosed,
                  width: 20,
                  height: 20,
                  color: 'red'
                },
                label: 'false',
                style: {
                  strokeWidth: '2px',
                  stroke: 'red'
                }
              });
            }
          }
        }

      } else if (json_x.instruction === 'goto') {
        edges.push({
          id: `e-${index}|${function_number}-goto`,
          source: `${index}|${function_number}`,
          target: `${ids[json_x.inputs[0]].index + 1}|${function_number}`,
          markerEnd: {
            type: MarkerType.ArrowClosed,
            width: 20,
            height: 20,
          },
          style: {
            strokeWidth: '2px',
          }
        });
      } else if (json_x.instruction !== 'exit_with_message' && json_x.instruction !== 'return') {
        // don't connect to next if next is goto since we bump back that connection
        if (indexs[index].instruction.instruction !== 'goto') {
          edges.push(constructEdge(index, function_number));
        }
      }
    });

  return edges;
};

function foobar(functions, supportedInstructions, supportedInstructionToColor, onUpdateFunctionName, onUpdateInputName) {
  let positionX = 0;

  return functions.map((f, i) => {
    const result = nodes(f, supportedInstructions, supportedInstructionToColor, i, onUpdateFunctionName, onUpdateInputName, positionX)
    const all_function_nodes = result.all_function_nodes;
    const last_width = result.last_width;
    positionX += last_width + 50;

    return all_function_nodes;
  }).flatMap(x => x)
}

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
              nodes={foobar(functions, supportedInstructions, supportedInstructionToColor, onUpdateFunctionName, onUpdateInputName)}
              edges={functions.map((f, i) => edges(f, i)).flatMap(x => x)}
              // fitView={{ padding: 1000 }}
              nodeTypes={nodeTypes}
              // panOnScroll={false}
              // zoomOnScroll={false}
              // zoomOnPinch={true}
              // nodesDraggable={false}
              // nodesConnectable={false}
              // elementsSelectable={false}
              minZoom={0.1}
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