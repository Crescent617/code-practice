////////////////////////////////////////10kV四端口两两隔离电能路由器仿真模型，三相完整模型，HVAC侧作为源////////

//仿真的情景模式为：单相HVAC侧向HVDC、LVDC、LVAC三个端口分别输送1/3*1/2MW、1/3*1/3MW、1/3*1/6MW的功率;

//HVAC侧控制：整流侧级联H桥控制模块电容电压，级联H桥采用载波移相调制，H桥自身采用双极性调制（S11与S14脉冲相同），未采用模块均压策略；高频H桥给定零移相比；
//HVDC侧控制：高频H桥控制模块电容电压，此处模块移相比相同，未考虑均压策略；
//LVDC侧控制：高频H桥控制模块电容电压；半桥控制正负375的电压；
//LVAC侧控制：高频H桥控制模块电容电压；三相四桥臂控制每相电压；

//////////////////////////////漏电阻参数

Lm=1e-3;
//Rm=inf;

//变压器漏电阻（HVAC/HVDC/LVDC端口变压器原副边漏感均为2e-8Ω）
//Rs1=46e-3; //高压30kva
//Rs2=20e-3; //高压80kva
//Rs3=31e-3; //低压80kva
//Rs4=0e-8;
//器件通态电阻
Ron=0.013*1;
Ron_inv=1.3e-3/4;

//变压器参数
R_TF_HA=50e-3;//HVAC短路电阻
L_TF_HA=45e-6;//HVAC短路电感
Lm_TF_HA=100;//HVAC磁化电感

R_TF_HD=50e-3;//HVDC短路电阻
L_TF_HD=35e-6;//HVDC短路电感
Lm_TF_HD=100;//HVDC磁化电感

R_TF_LD=9e-3;//LVDC短路电阻
L_TF_LD=16e-6;//LVDC短路电感
Lm_TF_LD=100;//LVDC磁化电感

//////////////效率测试/////////////
Pref_HD=500e3
Pref_LD=1000e3;
Pref_LA=700e3;

//////////////////////全局参数：
T_sim=1e-6;//仿真步长
T_duration=0.5;
f_DAB=20e3;// DAB所有开关器件的开关频率，这个也是所有高频H桥的采样频率；
P_DAB0=23.81e3;// 一个DAB模块的功率 HVAC侧模块;
P_DAB=66.67e3;// 一个DAB模块的功率 HVDC/LVDC/LVAC侧模块;
f_fundamental=50;//电网频率
//高频变压器的原副边电阻和漏感值分别为：2e-8、0;励磁电阻和电感值为：5e12、inf;

////////////////////////////////////////////HVAC侧电路和控制参数////////////////////////////////////////////
//主电路元器件命名规则：
//CHB的H桥：SiC11,SiC12,SiC13,SiC14;//高频H桥：SiC21,SiC22,SiC23,SiC24
//CHB的模块带电感：Lg/N;  //CHB的模块电容：C_HA;  //高频H桥的电感：L_HA;
//主电路参数：
Ul=10e3; //网侧线电压
Uph=Ul/sqrt(3); //网侧相电压
Psc_HA=20e3*10.5e3*sqrt(3);//端口短路容量
RsourceHA=Ul*Ul/Psc_HA/10;//电网内阻抗
LsourceHA=Ul*Ul/Psc_HA/314;
Lg=5.6e-3;//电网侧滤波电感，串联等效总电感
Rg=0.056;//串联电阻
N_HA=14; //HVAC侧单相级联模块的个数
C_HA=1000e-6; //HVAC侧模块的电容值
L_HA=46e-6;  //HVAC侧模块串联电感值，此处考虑了变压器漏感后的等效电感6uH；
UHA_ref=700; //HVAC侧电容参考电压
UHA_0=700;   //HVAC侧电容起始电压
//周期设置参数：
f_CHB=20/14*1e3;////HVAC侧CHB的开关频率20e3/14
T_sample_r=5e-05;//整流侧采样周期
//控制参数：
kp_CHB=0.03;//控制整流侧总的母线电压
ki_CHB=20;
kp_iac=112;//控制电网电流，无差拍；
kp_vc_i=0.04;
//三相时的控制参数；
kp_vc_HA=1;
ki_vc_HA=500;
kp_idq_HA=16;
ki_idq_HA=800;
////////////////////////////////////////////HVDC侧电路和控制参数////////////////////////////////////////////
//主电路元器件命名规则：
//高频H桥：SiC31,SiC32,SiC33,SiC34
//高频H桥的电感：L_HD;//模块电容：C_HD;//HVDC侧电阻：Rl_HD
N_HD=5; //HVDC侧单相串联模块的个数
L_HD=16e-6;  //HVDC侧模块串联电感值，此处考虑了变压器漏感后的等效电感2uH；
C_HD=1000e-6; //HVDC侧模块的电容值1000e-6
UHD_ref=666.7; //HVDC侧电容参考电压（666.7V）
UHD_0=UHD_ref;   //HVAC侧电容起始电压
Rl_HD=(UHD_ref*5*3 )^2/(Pref_HD/2);//三相时HVDC侧负载电阻（700改成666.7V）
R_HD = 5; //HVDC端口电阻
//控制参数：
kp_HD=0.01;//控制HVDC侧高频H桥电容电压
ki_HD=2;

//从机模式
Ll_HD=2.25e-3+5e-3;    //HVDC侧回路中串联的电感；
//Pref_HD=250e3;
//kp_HD_slave = 0.01;
//ki_HD_slave = 2;
////////////////////////////////////////////LVDC侧电路和控制参数////////////////////////////////////////////
//主电路元器件命名规则：
//高频H桥+半桥DC-DC：SiC41,SiC42,SiC43,SiC44,SiC45,SiC46
//高频H桥的电感：L_LD;//模块电容：C_LD1和C_LD2;
//主电路参数：
N_LD=5; //LVDC侧单相并联模块的个数
L_LD=16e-6;  //LVDC侧模块串联电感值
C_LD=1000e-6; //LVDC侧模块的电容值500e-6
L_DC=300e-6; //LVDC侧DC-DC环节的电感值
ULD_ref=750; //LVDC侧电容参考电压
ULD_0=750;   //LVDC侧电容起始电压
Rl_LD=(750)^2/(Pref_LD);//LVDC侧负载电阻
Ll_LD=0.15e-3+5e-3;
Rgound_LD=20;
//周期设置参数：
f_DC=20e3;  //LVDC中DC-DC环节的开关频率
T_sample_DC=50e-6;//整流侧采样周期
//LVDC侧控制参数
kp_LD=0.01;//控制LVDC侧高频H桥电容电压
ki_LD=1;
kp_DC=0.001;//控制LVDC侧高频H桥电容电压
ki_DC=0.01;

//从机模式
//Pref_LD=164e3;
//kp_LD_i=0.001;//控制LVDC侧直流电流
//ki_LD_i=0.1;
//R_LD = 0.02 * 1;  //LVDC端口电阻

////////////////////////////////////////////LVAC侧电路和控制参数////////////////////////////////////////////
//主电路元器件命名规则：
//高频H桥：SiC51,SiC52,SiC53,SiC54;
//三相四桥臂逆变：IGBT61,IGBT62,IGBT63,IGBT64,IGBT65,IGBT66,IGBT67,IGBT68
//高频H桥的电感：L_LA;//模块电容：C_LA;
//三相四桥臂的输出电感：Lf_LA_a,Lf_LA_b,Lf_LA_c,;输出电容：Cf_LA_a,Cf_LA_b,Cf_LA_c;
//负载电阻和电感：Rl_LA_a,Ll_LA_a;
//主电路参数：
N_LA=5; //LVAC侧单相并联模块的个数
L_LA=16e-6;  //LVAC侧模块串联电感值
//L_LA=L_LA/2;  //Plecs里电感分开布置，除以2
C_LA=1000e-6; //LVAC侧模块的电容值
ULA_ref=700; //LVAC侧电容参考电压
ULA_0=700;   //LVAC侧电容起始电压
Ul_LA_ref=380;//LVAC侧输出线电压
Uph_LA_ref=Ul_LA_ref/sqrt(3);//LVAC侧输出相电压
Lf_LA=54e-6;//LVAC侧滤波电感
Cf_LA=700e-6;//LVAC侧滤波电容
Ll_LA=1e-6;//LVAC侧负载电感
Ln_LA=18e-6;//LVAC侧中线电感
Rl_LA=(Uph_LA_ref)^2/(Pref_LA/3);//LVAC侧负载电阻
Ron_inv = 1.3e-3/4;
//周期设置参数：
f_INV=4e3;//5e3;  //LVAC侧INV的开关频率
T_sample_i=1/(f_INV*2);// 采样周期
//LVAC侧控制参数
kp_LA=0.01;//控制LVAC侧高频H桥的电容电压(0.01)
ki_LA=1;
kp_INV=2.8;//Cf_LA/T_sample_i/2;////PR控制器控制逆变侧电压
kp_iL=0.288;//(Lf_LA+Ln_LA)/T_sample_i/2;////无差拍控制器控制逆变输出电流
//Quasi-PR controller in INV
kr=350;//100
wc=5;//决定频率变化范围
w0=100*pi;
ALPHA=2*kr*wc;
BETA=2*kr*wc*exp(-wc*T_sample_i)*(cos(sqrt(w0^2-wc^2)*T_sample_i)+wc/sqrt(w0^2-wc^2)*sin(sqrt(w0^2-wc^2)*T_sample_i));
GAMMA=2*exp(-wc*T_sample_i)*cos(sqrt(w0^2-wc^2)*T_sample_i);
THETA=exp(-2*wc*T_sample_i);
//从机
//Pref_LA = 00e3; //LVAC发出功率给定值
//Iph_LA_ref = Pref_LA / 3 / Uph_LA_ref; //计算出LVAC输出电流给定值
//kp_INV_Slave = 0.3//PR控制器参数
//numz1_Slave = 0.1; //PR控制器参数，冲击响应不变法
//numz0_Slave = -0.0995; //PR控制器参数，冲击响应不变法
//denz1_Slave = 1.998458072481446; //PR控制器参数，冲击响应不变法
//R_LA_Slave = 0.02;//并网隔离变压器参数，或线路参数
//Ll_LA_Slave = 2e-6;
//numz=[0 0.062483937429329 -0.062483937429329];
//denz=[1 -1.998458072481449 1];

//numz=[500*T_sample_i -1*500*T_sample_i*cos(w0*T_sample_i) 0];
//denz=[1 -2*cos(w0*T_sample_i) 1];
//denz=[1 -1.9984580724814460 1];

//numz=[0.5 -0.49975328018287 0];

//// 改变初始参数
 UHA_0 = 700;  //// 2
 UHD_0 = 666.7;  //// 0
 ULD_0 = 750;  //// 0
 ULA_0 = 700;  //// 0

////////////////////////////HVAC充电电阻

Rcharge=250;

f_enable=1;
D_enable=0.5;  ////0.2
PD_enable=0;  ////0.07



DAB_sat_upper=0.5;
DAB_sat_lower=-0.5;

////////////////////////////////////////////模拟电压跌落
t_fall=0.1;
k_fall=1;

//////////////效率测试/////////////
Pref_HD=250e3
Pref_LD=164e3;
Pref_LA=660e3;
