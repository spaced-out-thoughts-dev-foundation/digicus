# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Aggregator::LoopAggregator do
  describe '.aggregate' do
    context 'when no loops' do
      let(:instructions) do
        [
          ins('add', %w[foo bar], 'a', 0),
          ins('sub', %w[foo bar], 'b', 0),
          ins('multiply', %w[foo bar], 'd', 1)
        ]
      end

      it 'returns the correct loop blocks' do
        expect(described_class.aggregate(instructions)).to eq(
          [
            { type: :instruction, instruction: ins('add', %w[foo bar], 'a', 0) },
            { type: :instruction, instruction: ins('sub', %w[foo bar], 'b', 0) },
            { type: :instruction, instruction: ins('multiply', %w[foo bar], 'd', 1) }
          ]
        )
      end
    end

    context 'when single loop' do
      let(:instructions) do
        [
          ins('instantiate_object', %w[Range 0 42], 'i', 0),
          ins('label', ['loop_top_0'], '', 0),
          ins('end_of_iteration_check', 'i', 'CHECK_CONDITION_ASSIGNMENT', 0),
          ins('goto', %w[CHECK_CONDITION_ASSIGNMENT loop_exit_0], '', 0),
          ins('increment', [], 'i', 0),
          ins('goto', ['loop_top_0'], '', 0),
          ins('label', ['loop_exit_0'], '', 0)
        ]
      end

      it 'returns the correct loop blocks' do
        # just check the loop
        expect(described_class.aggregate(instructions)[1]).to eq(
          { type: :loop, instructions: [
            { type: :instruction, instruction: ins('end_of_iteration_check', 'i', 'CHECK_CONDITION_ASSIGNMENT', 0) },
            { type: :instruction, instruction: ins('goto', %w[CHECK_CONDITION_ASSIGNMENT loop_exit_0], '', 0) },
            { type: :instruction, instruction: ins('increment', [], 'i', 0) },
            { type: :instruction, instruction: ins('goto', ['loop_top_0'], '', 0) }
          ] }
        )
      end
    end

    context 'when two loops' do
      let(:instructions) do
        [
          ins('instantiate_object', %w[Range 0 42], 'i', 0),
          ins('label', ['loop_top_0'], '', 0),
          ins('end_of_iteration_check', 'i', 'CHECK_CONDITION_ASSIGNMENT', 0),
          ins('goto', %w[CHECK_CONDITION_ASSIGNMENT loop_exit_0], '', 0),
          ins('increment', [], 'i', 0),
          ins('goto', ['loop_top_0'], '', 0),
          ins('label', ['loop_exit_0'], '', 0),
          ins('instantiate_object', %w[Range 0 42], 'j', 0),
          ins('label', ['loop_top_1'], '', 0),
          ins('end_of_iteration_check', 'j', 'CHECK_CONDITION_ASSIGNMENT', 0),
          ins('goto', %w[CHECK_CONDITION_ASSIGNMENT loop_exit_1], '', 0),
          ins('increment', [], 'j', 0),
          ins('goto', ['loop_top_1'], '', 0),
          ins('label', ['loop_exit_1'], '', 0)
        ]
      end

      it 'returns the correct loop blocks' do
        expect(described_class.aggregate(instructions)[0]).to eq(
          { type: :instruction, instruction: ins('instantiate_object', %w[Range 0 42], 'i', 0) }
        )

        expect(described_class.aggregate(instructions)[1]).to eq(
          { type: :loop, instructions: [{ type: :instruction, instruction: ins('end_of_iteration_check', 'i', 'CHECK_CONDITION_ASSIGNMENT', 0) },
                                        { type: :instruction,
                                          instruction: ins('goto', %w[CHECK_CONDITION_ASSIGNMENT loop_exit_0], '', 0) },
                                        { type: :instruction, instruction: ins('increment', [], 'i', 0) },
                                        { type: :instruction, instruction: ins('goto', ['loop_top_0'], '', 0) }] }
        )

        expect(described_class.aggregate(instructions)[2]).to eq(
          { type: :instruction, instruction: ins('instantiate_object', %w[Range 0 42], 'j', 0) }
        )

        expect(described_class.aggregate(instructions)[3]).to eq(
          { type: :loop, instructions: [{ type: :instruction, instruction: ins('end_of_iteration_check', 'j', 'CHECK_CONDITION_ASSIGNMENT', 0) },
                                        { type: :instruction,
                                          instruction: ins('goto', %w[CHECK_CONDITION_ASSIGNMENT loop_exit_1], '',
                                                           0) },
                                        { type: :instruction, instruction: ins('increment', [], 'j', 0) },
                                        { type: :instruction, instruction: ins('goto', ['loop_top_1'], '', 0) }] }
        )
      end
    end

    context 'when nested loops' do
      let(:instructions) do
        [
          ins('label', ['loop_top_0'], '', 0),
          ins('label', ['loop_top_1'], '', 0),
          ins('add', %w[1 2], '', 0),
          ins('label', ['loop_exit_1'], '', 0),
          ins('mul', %w[1 2], '', 0),
          ins('label', ['loop_exit_0'], '', 0)
        ]
      end

      it 'returns the correct loop blocks' do
        expect(described_class.aggregate(instructions)[0]).to eq(
          { type: :loop, instructions: [
            { type: :loop, instructions: [
              { type: :instruction, instruction: ins('add', %w[1 2], '', 0) }
            ] },
            { type: :instruction, instruction: ins('mul', %w[1 2], '', 0) }
          ] }
        )
      end
    end
  end
end
