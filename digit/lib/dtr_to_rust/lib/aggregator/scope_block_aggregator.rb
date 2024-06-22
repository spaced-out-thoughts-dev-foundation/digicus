# frozen_string_literal: true

module DTRToRust
  module Aggregator
    # Aggregates instructions into scope blocks
    class ScopeBlockAggregator
      def initialize(instructions)
        @instructions = instructions
      end

      def self.aggregate(instructions)
        new(instructions).aggregate
      end

      def aggregate
        sequential_scope_blocks = sequentially_group_by_scope(@instructions)

        decorate_scope_blocks(sequential_scope_blocks)
      end

      private

      def sequentially_group_by_scope(_instructions)
        return [] if @instructions.empty?

        scope_blocks = []
        current_scope = nil
        current_scope_block = []

        @instructions.each do |instruction|
          if current_scope == instruction.scope
            current_scope_block << instruction
          else
            scope_blocks << current_scope_block unless current_scope_block.empty?
            current_scope_block = [instruction]
            current_scope = instruction.scope
          end
        end

        scope_blocks << current_scope_block unless current_scope_block.empty?

        scope_blocks
      end

      def decorate_scope_blocks(scope_blocks)
        current_scope = nil
        current_scope_decorated_value = nil
        scope_memoize = {}

        scope_blocks.map do |scope_block|
          if current_scope.nil?
            current_scope = scope_block.first.scope
            current_scope_decorated_value = 0
            scope_memoize[current_scope] = current_scope_decorated_value
          elsif scope_memoize[scope_block.first.scope]
            current_scope = scope_block.first.scope
            current_scope_decorated_value = scope_memoize[current_scope]
          elsif current_scope < scope_block.first.scope
            current_scope = scope_block.first.scope
            current_scope_decorated_value += 1
            scope_memoize[current_scope] = current_scope_decorated_value
          else
            current_scope = scope_block.first.scope
            current_scope_decorated_value -= 1
            scope_memoize[current_scope] = current_scope_decorated_value
          end
          { block: scope_block, scope: current_scope, decorated_value: current_scope_decorated_value }
        end
      end
    end
  end
end
