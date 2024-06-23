# frozen_string_literal: true

require 'spec_helper'

RSpec.describe DTRToRust::Optimization::ChainedInvocationAssignmentReduction do
  describe '.apply' do
    context 'when the instructions are empty' do
      let(:instructions) { [] }

      it 'returns an empty array' do
        expect(described_class.apply(instructions)).to eq([])
      end
    end

    context 'when chained invocation assignment reduction is possible' do
      context 'when the instructions have no inputs' do
        let(:instructions) do
          [
            ins('evaluate', %w[size], 'BB', 0),
            ins('evaluate', ['BB.length'], 'CC', 0),
            ins('evaluate', ['CC.foobar'], 'd', 0)
          ]
        end

        it 'returns the optimized instructions' do
          expect(described_class.apply(instructions)).to eq([
                                                              ins('evaluate', ['size().length().foobar'], 'd', 0)
                                                            ])
        end
      end

      context 'when the instructions have inputs' do
        let(:instructions) do
          [
            ins('evaluate', %w[size 10], 'BB', 0),
            ins('evaluate', ['BB.length', '10', '20', '30'], 'CC', 0),
            ins('evaluate', ['CC.foobar'], 'd', 0)
          ]
        end

        it 'returns the optimized instructions' do
          expect(described_class.apply(instructions)).to eq([
                                                              ins('evaluate', ['size(10).length(10, 20, 30).foobar'],
                                                                  'd', 0)
                                                            ])
        end
      end
    end
  end
end

def ins(instruction, inputs, assign, scope)
  DTRCore::Instruction.new(instruction, inputs, assign, scope)
end
