#include<iostream>
#include<vector>
#include<string>
std::vector<char> info{'H','S','Y','N','O','A','D','S','I','Z','O','N','O','I','R','W','I','N','E','O','S','T','H','H'};//以H,S,Y,N,O为第一行，从此处开始，记录每个下标对应的字符
std::vector<std::vector<int>> edges{
	{5},{0,2},{1},{2,4},{3},
	{6},{10},{2,12},{7,9},{4,14},
	{5,15},{10,12},{8,16},{9,17},{13},
	{20},{15,17},{21,13},{23},{14},
	{16},{22},{18},{19},
};//以邻接表的形式记录有向图
void bfs(int node,int depth){
	static std::string stack(info.size(),0);
	static std::vector<bool> visited(info.size(),false);
	if(visited[node])
		return;
	visited[node]=true;
	stack[depth]=info[node];
	std::clog<<stack<<std::endl;
	if(depth==info.size()-1)
		std::cout<<stack<<std::endl;
	else
		for(int next:edges[node])
			bfs(next,depth+1);
	stack[depth]=0;
	visited[node]=false;
}//通过深度优先搜索，找寻所有可能的解
int main(){
	bfs(11,0);//通过观察，发现只有编号11的节点是没有入度的；因此，它必然是搜索的起点
	return 0;
}
