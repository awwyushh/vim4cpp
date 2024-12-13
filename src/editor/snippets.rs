use std::collections::HashMap;

pub fn default_snippets() -> HashMap<String, String> {
    let mut snippets = HashMap::new();
    
    snippets.insert("fio1".to_string(), 
r#"#include <bits/stdc++.h>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    
    return 0;
}"#.to_string());

    snippets.insert("fio2".to_string(),
r#"#include <bits/stdc++.h>
using namespace std;

const int BUFF_SIZE = 1 << 16;
char input_buffer[BUFF_SIZE];
int input_pos = 0, input_len = 0;

inline char next_char() {
    if (input_pos >= input_len) {
        input_pos = 0;
        input_len = fread(input_buffer, 1, BUFF_SIZE, stdin);
        if (input_len <= 0) return EOF;
    }
    return input_buffer[input_pos++];
}

inline int read_int() {
    int x = 0;
    char c = next_char();
    while (c <= ' ') c = next_char();
    bool neg = false;
    if (c == '-') {
        neg = true;
        c = next_char();
    }
    while (c >= '0' && c <= '9') {
        x = x * 10 + (c - '0');
        c = next_char();
    }
    return neg ? -x : x;
}

int main() {
    
    return 0;
}"#.to_string());

    snippets.insert("fio3".to_string(),
r#"#include <bits/stdc++.h>
using namespace std;

class FastIO {
private:
    static const int BUFF_SIZE = 1 << 16;
    char input_buffer[BUFF_SIZE];
    char output_buffer[BUFF_SIZE];
    int input_pos = 0, input_len = 0;
    int output_pos = 0;
    
public:
    inline char next_char() {
        if (input_pos >= input_len) {
            input_pos = 0;
            input_len = fread(input_buffer, 1, BUFF_SIZE, stdin);
            if (input_len <= 0) return EOF;
        }
        return input_buffer[input_pos++];
    }
    
    inline int read_int() {
        int x = 0;
        char c = next_char();
        while (c <= ' ') c = next_char();
        bool neg = false;
        if (c == '-') {
            neg = true;
            c = next_char();
        }
        while (c >= '0' && c <= '9') {
            x = x * 10 + (c - '0');
            c = next_char();
        }
        return neg ? -x : x;
    }
    
    inline void write_char(char c) {
        if (output_pos >= BUFF_SIZE) {
            fwrite(output_buffer, 1, output_pos, stdout);
            output_pos = 0;
        }
        output_buffer[output_pos++] = c;
    }
    
    inline void write_int(int x) {
        if (x < 0) {
            write_char('-');
            x = -x;
        }
        char digits[20];
        int len = 0;
        do {
            digits[len++] = x % 10 + '0';
            x /= 10;
        } while (x > 0);
        while (len > 0) {
            write_char(digits[--len]);
        }
    }
    
    ~FastIO() {
        if (output_pos) {
            fwrite(output_buffer, 1, output_pos, stdout);
        }
    }
} io;

int main() {
    
    return 0;
}"#.to_string());

    // Sorting Algorithms
    snippets.insert("bubblesort".to_string(),
r#"void bubbleSort(vector<int>& arr) {
    int n = arr.size();
    for(int i = 0; i < n-1; i++) {
        for(int j = 0; j < n-i-1; j++) {
            if(arr[j] > arr[j+1]) {
                swap(arr[j], arr[j+1]);
            }
        }
    }
}"#.to_string());

    snippets.insert("mergesort".to_string(),
r#"void merge(vector<int>& arr, int l, int m, int r) {
    int n1 = m - l + 1;
    int n2 = r - m;
    vector<int> L(n1), R(n2);
    
    for(int i = 0; i < n1; i++) L[i] = arr[l + i];
    for(int j = 0; j < n2; j++) R[j] = arr[m + 1 + j];
    
    int i = 0, j = 0, k = l;
    while(i < n1 && j < n2) {
        if(L[i] <= R[j]) arr[k++] = L[i++];
        else arr[k++] = R[j++];
    }
    while(i < n1) arr[k++] = L[i++];
    while(j < n2) arr[k++] = R[j++];
}

void mergeSort(vector<int>& arr, int l, int r) {
    if(l >= r) return;
    int m = l + (r - l) / 2;
    mergeSort(arr, l, m);
    mergeSort(arr, m + 1, r);
    merge(arr, l, m, r);
}"#.to_string());

    snippets.insert("quicksort".to_string(),
r#"int partition(vector<int>& arr, int low, int high) {
    int pivot = arr[high];
    int i = low - 1;
    
    for(int j = low; j < high; j++) {
        if(arr[j] < pivot) {
            i++;
            swap(arr[i], arr[j]);
        }
    }
    swap(arr[i + 1], arr[high]);
    return i + 1;
}

void quickSort(vector<int>& arr, int low, int high) {
    if(low < high) {
        int pi = partition(arr, low, high);
        quickSort(arr, low, pi - 1);
        quickSort(arr, pi + 1, high);
    }
}"#.to_string());

    snippets.insert("heapsort".to_string(),
r#"void heapify(vector<int>& arr, int n, int i) {
    int largest = i;
    int l = 2 * i + 1;
    int r = 2 * i + 2;
    
    if(l < n && arr[l] > arr[largest]) largest = l;
    if(r < n && arr[r] > arr[largest]) largest = r;
    
    if(largest != i) {
        swap(arr[i], arr[largest]);
        heapify(arr, n, largest);
    }
}

void heapSort(vector<int>& arr) {
    int n = arr.size();
    for(int i = n/2-1; i >= 0; i--)
        heapify(arr, n, i);
        
    for(int i = n-1; i > 0; i--) {
        swap(arr[0], arr[i]);
        heapify(arr, i, 0);
    }
}"#.to_string());

    // Searching Algorithms
    snippets.insert("binarysearch".to_string(),
r#"int binarySearch(vector<int>& arr, int target) {
    int left = 0, right = arr.size() - 1;
    
    while(left <= right) {
        int mid = left + (right - left) / 2;
        if(arr[mid] == target) return mid;
        if(arr[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;  // Not found
}"#.to_string());

    snippets.insert("lowerbound".to_string(),
r#"int lowerBound(vector<int>& arr, int target) {
    int left = 0, right = arr.size();
    
    while(left < right) {
        int mid = left + (right - left) / 2;
        if(arr[mid] < target) left = mid + 1;
        else right = mid;
    }
    return left;
}"#.to_string());

    // Graph Algorithms
    snippets.insert("dfs".to_string(),
r#"void dfs(vector<vector<int>>& graph, int v, vector<bool>& visited) {
    visited[v] = true;
    cout << v << " ";
    
    for(int u : graph[v]) {
        if(!visited[u]) {
            dfs(graph, u, visited);
        }
    }
}"#.to_string());

    snippets.insert("bfs".to_string(),
r#"void bfs(vector<vector<int>>& graph, int start) {
    int n = graph.size();
    vector<bool> visited(n, false);
    queue<int> q;
    
    visited[start] = true;
    q.push(start);
    
    while(!q.empty()) {
        int v = q.front();
        q.pop();
        cout << v << " ";
        
        for(int u : graph[v]) {
            if(!visited[u]) {
                visited[u] = true;
                q.push(u);
            }
        }
    }
}"#.to_string());

    snippets.insert("dijkstra".to_string(),
r#"vector<int> dijkstra(vector<vector<pair<int,int>>>& graph, int start) {
    int n = graph.size();
    vector<int> dist(n, INT_MAX);
    priority_queue<pair<int,int>, vector<pair<int,int>>, greater<>> pq;
    
    dist[start] = 0;
    pq.push({0, start});
    
    while(!pq.empty()) {
        int d = pq.top().first;
        int v = pq.top().second;
        pq.pop();
        
        if(d > dist[v]) continue;
        
        for(auto [u, weight] : graph[v]) {
            if(dist[v] + weight < dist[u]) {
                dist[u] = dist[v] + weight;
                pq.push({dist[u], u});
            }
        }
    }
    return dist;
}"#.to_string());

    // Data Structures
    snippets.insert("dsu".to_string(),
r#"class DSU {
    vector<int> parent, rank;
public:
    DSU(int n) : parent(n), rank(n, 0) {
        for(int i = 0; i < n; i++) parent[i] = i;
    }
    
    int find(int x) {
        if(parent[x] != x) parent[x] = find(parent[x]);
        return parent[x];
    }
    
    void unite(int x, int y) {
        int px = find(x), py = find(y);
        if(px == py) return;
        if(rank[px] < rank[py]) swap(px, py);
        parent[py] = px;
        if(rank[px] == rank[py]) rank[px]++;
    }
}"#.to_string());

    snippets.insert("segtree".to_string(),
r#"class SegTree {
    vector<int> tree;
    int n;
public:
    SegTree(vector<int>& arr) {
        n = arr.size();
        tree.resize(4 * n);
        build(arr, 0, 0, n-1);
    }
    
    void build(vector<int>& arr, int node, int start, int end) {
        if(start == end) {
            tree[node] = arr[start];
            return;
        }
        int mid = (start + end) / 2;
        build(arr, 2*node+1, start, mid);
        build(arr, 2*node+2, mid+1, end);
        tree[node] = tree[2*node+1] + tree[2*node+2];
    }
    
    void update(int idx, int val) { update(0, 0, n-1, idx, val); }
    int query(int l, int r) { return query(0, 0, n-1, l, r); }
    
private:
    void update(int node, int start, int end, int idx, int val) {
        if(start == end) {
            tree[node] = val;
            return;
        }
        int mid = (start + end) / 2;
        if(idx <= mid) update(2*node+1, start, mid, idx, val);
        else update(2*node+2, mid+1, end, idx, val);
        tree[node] = tree[2*node+1] + tree[2*node+2];
    }
    
    int query(int node, int start, int end, int l, int r) {
        if(r < start || end < l) return 0;
        if(l <= start && end <= r) return tree[node];
        int mid = (start + end) / 2;
        return query(2*node+1, start, mid, l, r) +
               query(2*node+2, mid+1, end, l, r);
    }
}"#.to_string());

    snippets
}
