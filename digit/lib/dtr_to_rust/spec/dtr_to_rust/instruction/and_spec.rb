# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::And do
  describe '#handle' do
    let(:instruction) { DTRCore::Instruction.new('and', %w[foo bar], 'LOGICAL_RESULT', 0) }

    it 'returns the correct string' do
      expect(described_class.handle(instruction, [], [], false)).to eq('        let LOGICAL_RESULT = foo && bar;')
    end
  end
end
