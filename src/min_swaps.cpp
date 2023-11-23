#include <iostream>
#include <vector>
#include <cassert> // For assert

using std::cout;
using std::endl;
using std::vector;

int minimum_swaps(vector<int> arr);
int quicksort_and_count(vector<int> &arr, int low, int high);
int partition_and_count(vector<int> &arr, int low, int high, int &swap_count);

int minimum_swaps(vector<int> arr)
{
  return quicksort_and_count(arr, 0, arr.size());
}

// Complete the minimumSwaps function below.
int quicksort_and_count(vector<int> &arr, int low, int high)
{
  int swap_count = 0;
  if (low < high)
  {
    int pivot_index = partition_and_count(arr, low, high, swap_count);
    // int pivot_index = partition(arr, low, high, &swap_count);
    swap_count += quicksort_and_count(arr, low, pivot_index);
    swap_count += quicksort_and_count(arr, pivot_index + 1, high);
  }
  return swap_count;
}

int partition_and_count(vector<int> &arr, int low, int high, int &swap_count)
{
  int pivot_value = arr[high];
  int i = low - 1;
  for (int j = low; j < high; j++)
  {
    if (arr[j] < pivot_value)
    {
      i++;
      if (i != j)
      {
        std::swap(arr[j], arr[i]);
        swap_count++;
      }
    }
  }
  return (i + 1);
}

void run_tests()
{
  // Test case 1: Already sorted array
  vector<int> arr1 = {1, 2, 3, 4, 5};
  assert(minimum_swaps(arr1) == 0);

  // Test case 2: Reverse sorted array
  vector<int> arr2 = {5, 4, 3, 2, 1};
  assert(minimum_swaps(arr2) == 2); // This can vary based on pivot selection

  // Test case 3: Random order
  vector<int> arr3 = {2, 3, 1, 5, 4};
  assert(minimum_swaps(arr3) == 3); // This can vary based on pivot selection

  // Test case 4: All elements are the same
  vector<int> arr4 = {1, 1, 1, 1};
  assert(minimum_swaps(arr4) == 0);

  cout << "All tests passed successfully." << endl;
}

int main()
{
  run_tests();
  return 0;
}
