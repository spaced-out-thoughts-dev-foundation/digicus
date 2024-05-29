export function supportedInstructionToColor (supported_instruction) {
  if (supported_instruction == null || supported_instruction.category == null) {
    return 'white';
  }
  if (supported_instruction.category === "basic") {
    return 'orange';
  } else if (supported_instruction.category === "state") {
    return 'red';
  } else if (supported_instruction.category === "untyped") {
    return 'silver';
  } else if (supported_instruction.category === "numeric") {
    return 'lavender';
  } else if (supported_instruction.category === "string") {
    return 'yellow';
  } else if (supported_instruction.category === "environment") {
    return 'turquoise';
  } else if (supported_instruction.category === "methods") {
    return 'pink';
  } else if (supported_instruction.category === "objects") {
    return 'skyblue';
  } else {
    return 'white';
  }
};