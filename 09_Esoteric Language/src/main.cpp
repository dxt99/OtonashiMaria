#include<bits/stdc++.h>
using namespace std;

vector<string> code;
vector<string> ops;
int blockCount;
int lastBlock;
int buff[135];
int idx=1;
int* pointer=&buff[idx];
int mode=0; // 0 = element, 1 = pointer

bool is135(string s){
	int n = s.size();
	if(n<5)return 0;
	return s.substr(n-4, 4) == ".135";
}

bool isInt(char c){
	return c=='1' || c=='3' || c=='5';
}

bool isWhite(char c){
	return c==' ' || c=='\n' || c=='\r' || c=='\t';
}

bool isOp(string s){
	vector<string> ops = {"**", "+", "-", "*", "/", "&", "^", "|", "%"};
	for(string k: ops)if(k == s)return 1;
	return 0;
}

int count(int op1, string op, int op2){
	if(op == "+")return op1+op2;
	if(op == "-")return op1-op2;
	if(op == "*")return op1*op2;
	if(op == "/")return op1/op2;
	if(op == "&")return op1&op2;
	if(op == "|")return op1|op2;
	if(op == "%")return op1%op2;
	if(op == "^")return op1^op2;
	if(op == "**")return pow(op1,op2);
	return -1;
}

bool lineLint(string line, int cnt){
	int op1, op2;
	string op;
	int i = 0;
	string temp = "";
	while(isWhite(line[i])||isInt(line[i])){
		if(!isWhite(line[i]))temp += line[i];
		i++;
		if(i>line.size()) break;
	}
	if(temp.size()==0 && i<line.size()){
		cout<<"Syntax error at line "<<cnt<<endl;
		return 0;
	}
	if(temp.size()>0)op1 = stoi(temp);
	else op1 = 135;
	while(i<line.size()){
		temp = "";
		while(!isInt(line[i])){
			if(!isWhite(line[i]))temp += line[i];
			i++;
			if(i>line.size()) break;
		}
		if(!isOp(temp)){
			cout<<"Syntax error at line "<<cnt<<endl;
			return 0;
		}
		op = temp;
		ops.push_back(op);
		if(op == "^"){
			blockCount++;
			lastBlock = cnt;
		}
		temp = "";
		while(isWhite(line[i])||isInt(line[i])){
			if(!isWhite(line[i]))temp += line[i];
			i++;
			if(i>line.size()) break;
		}
		if(temp.size()==0){
			cout<<"Syntax error at line "<<cnt<<endl;
			return 0;
		}
		op2 = stoi(temp);
		op1 = count(op1, op, op2);
	}
	if(op1 != 135){
		cout<<"135 error at line "<<cnt<<endl;
		return 0;
	}
	return 1;
}

bool compile(){
	int cnt = 1;
	for(string line: code){
		if(!lineLint(line, cnt))return 0;
		cnt++;
	}
	if(blockCount%2){
		cout<<"Unmatched block: last block started at line "<<lastBlock<<" and not yet closed"<<endl;
		return 0;
	}
	return 1;
}


bool command(string op){
	if(op == "+")(*pointer)++;
	else if(op == "-")(*pointer)--;
	else if(op == "*")(*pointer)<<=1;
	else if(op == "/")(*pointer)>>=1;
	else if(op == "&")cout<<(char)buff[idx];
	else if(op == "|"){
		string s; cin>>s;
		if(s.size()>135){
			cout<<"Runtime error: buffer overflow"<<endl;
			return 0;
		}
		int i=idx;
		for(int j=0; j<s.size(); j++){
			buff[i]=s[j];
			i++;
			i%=135;
		}
	}
	else if(op == "**"){
		mode = mode^1;
		if(mode)pointer=&idx;
		else pointer=&buff[idx%135];
	}
	if(mode){
		while((*pointer)<0)(*pointer)+=135;
		if((*pointer)>135)(*pointer)%=135;
		if((*pointer)==0)(*pointer)=135;
	}else{
		if((*pointer)<0||(*pointer)>255){
			cout<<"Runtime error: char out of bounds"<<endl;
			return 0;
		}
	}
	return 1;
}

void run(){
	for(int i=0; i<ops.size(); i++){
		string op = ops[i];
		if(op == "%"){
			if(buff[idx]==135)continue;
			if(i==ops.size()-1)continue;
			i++;
			if(ops[i]!="^")continue;
			i++;
			while(ops[i]!="^")i++;
			continue;
		}
		if(op != "^"){
			if(!command(op))return;
			continue;
		}	
		int j=i+1;
		i++;
		while(ops[i]!="^")i++;
		while(buff[0]!=0){
			for(int k=j; k<i; k++)if(!command(ops[k]))return;
		}
	}
}

int main(int argc, char** argv){
	for(int i=0; i<135; i++)buff[i]=0;
	if(argc != 2 || !is135(argv[1])){
		cout<<"Usage: ./135.exe {filename}.135"<<endl;
		return 0;
	}
	ifstream file;
	file.open(argv[1]);
	string temp;
	while(getline(file,temp))code.push_back(temp);
	if(!compile())return 0;
	run();
	return 0;
}
