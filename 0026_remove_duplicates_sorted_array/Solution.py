class Solution(object):
    def removeDuplicates(self, nums):
        n = len(nums)
        if n == 0:
            return 0

        write = 1
        for read in range(1, n):
            if nums[read] != nums[write - 1]:
                nums[write] = nums[read]
                write += 1
        return write


if __name__ == "__main__":
    solution = Solution()
    print(solution.removeDuplicates([1, 1, 2, 2, 3]))
    print(solution.removeDuplicates([0,0,1,1,1,2,2,3,3,4]))
    print(solution.removeDuplicates([]))
