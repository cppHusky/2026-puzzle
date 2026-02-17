#!/bin/sage
def calc_alphas(c1,c2,c3):#Ensure c1>=c2>=c3
	alpha_1=c3/255.;
	c2_1=(c2-c3)/(1-alpha_1);
	c1_1=(c1-c3)/(1-alpha_1);
	alpha_2=c2_1/255.;
	c1_2=(c1_1-c2_1)/(1-alpha_2);
	alpha_3=c1_2/255.;
	return [alpha_3,alpha_2,alpha_1];
def breakdown_color(r,g,b):
	items=[("red",r),("green",g),("blue",b)];
	items=sorted(items,key=lambda kv:kv[1],reverse=True);
	alphas=calc_alphas(items[0][1],items[1][1],items[2][1]);
	accumulated=[];
	current=[];
	for name,alpha in zip([name for name,_ in items],alphas):
		current.append(name);
		accumulated.append(("+".join(current),alpha));
	return accumulated;
print(breakdown_color(0x20,0x35,0xf0));
