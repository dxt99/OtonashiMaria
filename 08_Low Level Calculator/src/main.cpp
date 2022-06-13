#include<bits/stdc++.h>
#include "op.hpp"
using namespace std;

std::vector<std::string> split(const std::string & s, char c)
{
    std::vector<std::string> splitted;

    std::string word;
    for(char ch : s)
    {
        if((ch == c) && (!word.empty()))
        {
            splitted.push_back(word);
            word.clear();
        }
        else
            word += ch;
    }
    if(!word.empty())
        splitted.push_back(word);

    return splitted;
}

int main(){
	int choice = 1;
	while(choice!=3){
		cout<<"Pilih mode operasi:"<<endl;
		cout<<"1. Aritmetika (+,-,*,/,^)"<<endl;
		cout<<"2. Inverse Squareroot (1/sqrt(x))"<<endl;
		cout<<"3. Exit"<<endl;
		cout<<">> ";
		cin>>choice;
		switch(choice){
			case 1:{ // Aritmetika
					cout<<"Masukkan Ekspresi: "<<endl;
					string a;
					int op1, op2;
					string op;
					int cnt=0;
					string b;
					getline(cin, b);
					getline(cin, b);
					for(const std::string & a : split(b, ' ')){
						if(cnt==0){
							op1 = stoi(a);
							cnt++;
							continue;
						}
						if(cnt%2){
							op = a;
							cnt++;
							continue;
						}
						op2 = stoi(a);
						if(op=="+")	op1 = add(op1, op2);
						else if(op=="-") op1 = subtract(op1, op2);
						else if(op=="*") op1 = multiply(op1, op2);
						else if(op=="/") op1 = divide(op1, op2);
						else if(op=="^") op1 = power(op1, op2);
						else cout<<"Invalid operation"<<endl;
						cnt++;
					}
					cout<<"Hasil: "<<op1<<endl;
					break;
				}
			case 2:{
					cout<<"Masukkan angka (float): ";
					float a; cin>>a;
					cout<<"Hasil: "<<invsqrt(a)<<endl;
					break;
				}
			case 3:
				cout<<"Exiting ..."<<endl;
				break;
			default:
				cout<<"Pilihan tidak valid"<<endl;
				break;
		}		
	}
}
