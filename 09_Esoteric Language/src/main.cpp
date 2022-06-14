#include<bits/stdc++.h>
using namespace std;

bool is135(string s){
	int n = s.size();
	if(s.size<5)return 0;
	return s.substr(n-4, 4) == ".135";
}

int main(int argc, char** argv){
	if(argc != 1 || !is135(argv[0])){
		cout<<"Usage: ./135.exe {filename}.135"<<endl;
		return 0;
	}
	ifstream file;
	file.open(argv[0]);
	vector<string> code;
	string temp;
	while(getline(file,temp))code.push_back(temp);
	for(string s: code)cout<<s<<endl;
}
