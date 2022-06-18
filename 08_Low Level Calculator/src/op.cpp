int add(int a, int b){ 
	addloop:
	int carry = a&b;
	a = a^b;
	b = carry<<1;
	if(b!=0) goto addloop;
	return a;
}

int subtract(int a, int b){ //returns a-b
	b = ~b;
	b = add(b,1); //two's complement
	return add(a,b);
}

int abs(int x){ //returns abs(x)
	if(x>=0) return x;
	x = ~x;
	return (add(1,x));
}

int multiply(int a, int b){ //returns a*b
	int ret = 0;
	int sign = (a&1<<31) ^ (b&1<<31);
	a = abs(a); b = abs(b);
	multiplyloop:
	if(b&1)ret = add(a,ret);
	a<<=1;
	b>>=1;
	if(b!=0)goto multiplyloop;
	if(sign){
		ret = ~ret;
		return(add(1,ret));
	}
	return ret;
}


int divide(int a, int b){ //returns a/b
	int sign = (a & 1<<31) ^ (b&1<<31);
	a = abs(a); b = abs(b);
	int ret = 0;
	divideloop:
	ret = add(ret, 1);
	a = subtract(a,b);
	if(a>=b) goto divideloop;
	if(sign){
		ret = ~ret;
		return(add(1,ret));
	}
	return ret;
}

int power(int a, int b){ //returns a^b
	if(b<0)return 0;
	int ret = 1;
	powerloop:
	if(b&1)ret=multiply(ret, a);
	a = multiply(a,a);
	b >>= 1; 
	if(b!=0) goto powerloop;
	return ret;
}

float invsqrt(float number){ //returns 1/(sqrt(a)), quake 3 alg
	long i;
	float x2,y;
	const float threehalfs = 1.5F;
	
	x2 = number * 0.5F;
	y = number;
	i = *(long*)&y;
	i = 0x5f3759df - (i>>1);
	y = *(float*) &i;
	y = y*(threehalfs-(x2*y*y));
	
	return y;
}
