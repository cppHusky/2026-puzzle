#!/bin/wolframscript -print all
c=Cycles@Table[{10i+#1,10i+#2},{i,5}]&;
r=Cycles@Table[{10#1+i,10#2+i},{i,5}]&;
ex=Cycles@{{#1,#2}}&;
Inv=Cycles@(Reverse@#&/@First@#)&;
P=PermutationProduct;
op1=r[1,2];
op2=P[ex[13,22],c[2,3]];
op3=ex[12,21];
P[op1,op2,op3,Inv@op2,Inv@op1]
