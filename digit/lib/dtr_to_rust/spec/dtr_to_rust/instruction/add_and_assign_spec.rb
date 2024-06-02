# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::AddAndAssign do
  describe '#handle' do
    it 'returns the correct Rust code' do
      instruction = {
        instruction: 'add_and_assign',
        inputs: ['count', 1],
        assign: nil,
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('count += 1;')
    end
  end
end
