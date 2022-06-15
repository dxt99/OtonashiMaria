#include <iostream>
#include <windows.h>
#include <stdio.h>
#include <tchar.h>
#include <psapi.h>

using namespace std;

DWORD tarPid;
TCHAR tarName[MAX_PATH] = TEXT("target.exe");
long long int value = 0x135;
long long int address;

void print_ps_name(DWORD pid) {
    TCHAR pname[MAX_PATH] = TEXT("<unknown>");
    HANDLE phandle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
    if (phandle != NULL) {
        HMODULE pmod;
        DWORD temp;
        if (EnumProcessModules(phandle, &pmod, sizeof(pmod), &temp))
            GetModuleBaseName(phandle, pmod, pname, sizeof(pname) / sizeof(TCHAR));
    }
    //_tprintf(TEXT("%s (PID %u)\n"), pname, pid);
    if(_tcscmp(tarName, pname)==0)tarPid = pid;
    CloseHandle(phandle);
}

void print_ps() {
    DWORD ps_list[1024], ps_memctr, ps_ctr;
    if (!EnumProcesses(ps_list, sizeof(ps_list), &ps_memctr))
        return;
    ps_ctr = ps_memctr / sizeof(DWORD);
    for (int i = 0; i < ps_ctr; i++)
        if (ps_list[i] != 0)
            print_ps_name(ps_list[i]);
}


int main() {
    print_ps();
    HANDLE hProc = OpenProcess(PROCESS_ALL_ACCESS, false, tarPid);
    if(!hProc) {
        cerr << "Cannot open process." << endl;
    } else {
        cout << "Enter target address (hex): ";
        cin >> hex >> address;
        cout << "Change value to (hex): ";
        cin >> hex >> value;
        int stat = WriteProcessMemory(hProc, (LPVOID)address, &value, (DWORD)sizeof(value), NULL);

        if(stat > 0){
            clog << "Memory written to process." << endl;
        } else {
            cerr << "Memory couldn't be written to process." << endl;
        }
        CloseHandle(hProc);
    }
    return 0;
}
