# frozen_string_literal: true

require 'spec_helper'

RSpec.describe DTRToRust::Optimization::FieldToAssignmentConversion do
  describe '.apply' do
    context 'when the instructions are empty' do
      let(:instructions) { [] }

      it 'returns an empty array' do
        expect(described_class.apply(instructions)).to eq([])
      end
    end

    context 'when field can be converted to an assign' do
      let(:instructions) do
        [
          ins('field', %w[state count], 'STATE_COUNT', 0),
          ins('add', %w[STATE_COUNT 1], 'STATE_COUNT', 0)
        ]
      end

      let(:expected_instructions) do
        [
          ins('assign', ['state.count'], 'STATE_COUNT', 0),
          ins('add', %w[STATE_COUNT 1], 'STATE_COUNT', 0)
        ]
      end

      it 'returns the optimized instructions' do
        expect(described_class.apply(instructions)).to eq(expected_instructions)
      end
    end
  end
end

def ins(instruction, inputs, assign, scope)
  DTRCore::Instruction.new(instruction, inputs, assign, scope)
end
