module Main exposing (..)

import DayOne
import DayTwo
import DayThree
import DayFour
import DayFive
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)


---- MODEL ----


type alias Model =
    { dayOneInput : String
    , dayTwoInput : String
    , dayThreeInput : String
    , dayFourInput : String
    , dayFiveInput : String
    }


init : ( Model, Cmd Msg )
init =
    ( Model
        "951484596541141557316984781494999179679767747627132447513171626424561779662873157761442952212296685573452311263445163233493199211387838461594635666699422982947782623317333683978438123261326863959719777179228599319321138948466562743761584836184512984131635354116264899181952748224523953976485816295227945792555726121913344959454458829485471174415775278865324142733339789878929596275998341778873889585819916457474773252249179366599951454182657225576277834669222982366884688565754691273745959468648957498511326215934353963981471593984617554514519623785326888374742147318993423214834751785956958395133486656388454552769722562524415715913869946325551396638593398729938526424994348267935153555851552287223313383583669912941364344694725478258297498969517632881187394141593479818536194597976519254215932257653777455227477617957833273463216593642394215275314734914719726618923177918342664351954252667253233858814365351722938716621544226598956257753212248859258351363174782742336961425325381561575992352415514168782816173861148859478285339529151631429536819286498721812323861771638574344416879476255929929157912984151742613268754779685396125954595318134933366626594498249956388771723777242772654678448815844555372892574747735672368299826548254744359377667294764559334659523233146587568261116253155189394188696831691284711264872914348961888253386971994431352474717376878745948769171243242621219912378731755544387249443997382399714738351857752329367997665166956467544459817582915478514486541453932175598413554259672117364863112592515988922747164842668361925135551248923449968328385889877512156952725198691746951431443497496455761516486573476185321748523644283494181119399874324683922393547682851931435931276267766772798261563117954648576421741384823494187895272582575669685279986988357796138794326125852772995446355723211161523161886222562853546488411563473998633847953246787557146187696947831335722888918172961256498971868946237299523474841983527391489962357196433927251798764362493965894995592683296651874787384247326643886774966828657393717626591578321174832222434128817871765347278152799425565633521152643686221411129463425496425385516719682884157452772141585743166647191938727971366274357874252166721759"
        """
62 1649 1731 76 51 1295 349 719 52 1984 2015 2171 981 1809 181 1715
161 99 1506 1658 84 78 533 242 1685 86 107 1548 670 960 1641 610
95 2420 2404 2293 542 2107 2198 121 109 209 2759 1373 1446 905 1837 111
552 186 751 527 696 164 114 530 558 307 252 200 481 142 205 479
581 1344 994 1413 120 112 656 1315 1249 193 1411 1280 110 103 74 1007
2536 5252 159 179 4701 1264 1400 2313 4237 161 142 4336 1061 3987 2268 4669
3270 1026 381 185 293 3520 1705 1610 3302 628 3420 524 3172 244 295 39
4142 1835 4137 3821 3730 2094 468 141 150 3982 147 4271 1741 2039 4410 179
1796 83 2039 1252 84 1641 2165 1218 1936 335 1807 2268 66 102 1977 2445
96 65 201 275 257 282 233 60 57 200 216 134 72 105 81 212
3218 5576 5616 5253 178 3317 6147 5973 2424 274 4878 234 200 4781 5372 276
4171 2436 134 3705 3831 3952 2603 115 660 125 610 152 4517 587 1554 619
2970 128 2877 1565 1001 167 254 2672 59 473 2086 181 1305 162 1663 2918
271 348 229 278 981 1785 2290 516 473 2037 737 2291 2521 1494 1121 244
2208 2236 1451 621 1937 1952 865 61 1934 49 1510 50 1767 59 194 1344
94 2312 2397 333 1192 106 2713 2351 2650 2663 703 157 89 510 1824 125
"""
        "312051"
        """
bdwdjjo avricm cjbmj ran lmfsom ivsof
mxonybc fndyzzi gmdp gdfyoi inrvhr kpuueel wdpga vkq
bneh ylltsc vhryov lsd hmruxy ebnh pdln vdprrky
fumay zbccai qymavw zwoove hqpd rcxyvy
bcuo khhkkro mpt dxrebym qwum zqp lhmbma esmr qiyomu
qjs giedut mzsubkn rcbugk voxk yrlp rqxfvz kspz vxg zskp
srceh xdwao reshc shecr
dcao isz wwse bbdgn ewsw qkze pwu
lbnvl lviftmr zqiv iadanl fdhrldn dlaani lxy dhfndrl fkoukx
raovmz pdysjsw hqps orsyqw rrwnzcz vrzoam jjljt
wgt gzi icpwp qeefgbe msadakj jbbrma sbj dufuujx zex
cfzx bvyu eswr hafcfy klw bgnhynv qrf aop
rzlq atrzcpb hpl pajjw cdxep ean aptzcrb rzcbapt
xogpf ucc nsukoz umtfbw xfvth ozusnk fopxg ubp iflb
xot nqcdyu kpwix szo cyxv hpmp hwtrc zso nyuqdc aha
mkzf cat tkjprc izxdggf obspan lmlbg bsyspf twox
lfmfrd ooclx tcl clt
dxvnyd nxwojj arutn eyqocj swzao tmh juvpezm
teu eman rlmdmk xkbodv fvrcm zorgy wmwe
hmo fdayx duciqf cgt duciqf
imjnv vfmsha cyrusow xjswoq nclrmjy sjxowq ynjrcml
rwbsay alsi bmzpvw ozq aduui nihwx glwdiz ixmkgfx
vtjzc ntkh zekj qrbkjhn zekj lyfnbg
afaig jqhli oie lhwyduh kqfnraz nfrzaqk mayfg iljqh
inb zum zmu dnl zjxg vrdziq ypdnsvt
uhbzmre mpdxm alkbmsq aopjmkl mqxenry ayvkrf zxvs qkfqva
fimjr ccv cnug crdsv
bqyve lhxdj ydu qbyve vihwjr vyodhc
vmng dyttyf noagpji tdtyfy ewiest ogg
kgscfj klmsv vmksl vmlks
qlvh veo wruft wtm fbrlvjr evo wvwmny dhp bvrlrfj lvt vgzuyyw
mxuro orxmu tivu tjdq ojjvju cdd
kjexme gxbysxp yxrum hinrklv fxehytd qkt tqk umryx nim
kywnux wab tzrft dsaz jgwuw dubarmi fle wjguvr umjp uzncwj mzz
qokwh zrda xywufk tbxhhj eejqaoa hwoqk zer hwt hbjxth xyf hmh
eregs qdx tdequa agrlrg mwwpba qjie yrjvhr txujk
iyot fxwdcb zvwfv vfzwv wvkw ncwbr wdejrr ltcdza
waix eza znmonya ldfghws ialwfvc dey ubsz uhbnh svgekg nonzyam
bryz tfbqn xznfmw xiplgww wwxigpl jxzcgxl rzyb
cqvl rrcoqxs staeuqr hzzow cwv tsvol dio coc ddavii uuojy
txbn qvkkyh gbqnjtq ndpkqr srt bkpqfmm ytycev ypcv bpqmmfk
uqkjmul dour zgq ztango yrrjhrg ufxnmuw
ekxbcv vkxbec xbcevk jiq bar
wczff qdu cwffz hhk wlvyg
zjlconc osgsro dajzo hqih ehml
hnio shccluw cpu ivaby tormn vkef abv vkef ivaby
xgbdeso xiizs omqwy sbtnnt khago evviclw xyu dtvg wsyxfuc humewp
cnzu bia vdyqrf wwb qveum hmh ouupgc owli
pjpmfxa dvd lxgh ndy gwph oebfkqv vtlxdg efl ekj dyn
mvan nmdkc ucyshia mavn ecst poo
oybm pjwm bmyo wovgu xykziaq obmy eiirhqd
xkvomx yxvv oxxpth elh vxvy lhe ycn
okxglw gmaangx gnxaamg yduzrr nzwxtnd rcxcu xjjvno yat cin gaxnamg yss
oicgs rrol zvnbna rrol
abb edpnxuo peoudxn bab ceay
ncpkfz gvwunb fckpzn caafx pkcfzn tsfl
fnrt ymenkpq wodubcm niv nvi ziluu cuowbdm zocg pdakwt mlzxkex nuxqclo
uouxcgl stgua otadr ideannq wizxunv iqsdpj mxddt ldst ucxogul
rbrwyhk wqoz zqwo ikwgexl atpu iza
smo yolp pcahlu muljxkq cbkljmz zlbcmkj zvbmgz eaiv ncv zplifm yplo
ocutdhz zmnaap llgv llzpl loavju guzkfq saay rxyhng cwxzx lcv anrnzs
etyzx tcm upxrtvd imyoiu rdpj fed dmm
gonqa szteh szteh razdqh phyff upf knfqfaf knfqfaf fpsgl kakag
mcju mixskop isrwat lcr nfyi lcr aaevr nfyi pqrbk gnful
xfmr fkmnq fbnhd mxrf litniid xbae frxm zcenf
yuh lzojtj rqsh hyu
vbjgql yeshsuv lokt efqota wpwjfu ykyq rxc fxxh ycqfkk gndts vdf
wnylmr kkuruxm azr xukrkum dohkwx dmdb
bjiyrwf dvf fdv vdf gnokekr
jsaq hcww iayqtu llv gdpxdrd hwlo nosjit wpm lcab fcgwr
fxjp bys nnf xzllckh bys hvojw zcwtgwz wye ccyvjv
grafa hbb ghk wkdpsf ufa uoqmysd
yvacf kssbff iovrm mvrio cfbpb avh zzje
gqd qmsen wkvrfz vhtsa zrwfkv gul zkvwrf
hrbi svaogb aogsvb bgrk hibr jbtkr
ljl ryc mrewrge yky
fcqyymt emk qcmyytf mcfvusb luy qany cbsvumf
oknt mcozuc ccmuoz uoccmz
uziil xobutwf acnof iqgwl diso
sekq fxbtsuv ddnnqg rnemlt dngnqd hhgjfus stxvubf
lajcp qgiw fyz blrlcd pviwv buh wnkk
wolqfk nvpapfc rwcqxfz xobno yzjfz irpj wolqfk wbnwjt
vmabj noiljne hhqf holxkbk swwzx ylgj lnmxy lqodhk abjvm bmsrf
bpnp yrz pjepxxs jlmhuy vihlx zacm inuazhc xsxjepp
tryl kryh eonvaad ucevssk umkxg lqej nswopjj svkeucs bmh stosxxz
cfdwd dmfdrvm ibuhsz nwtgmb pjt dmfdrvm cqdcm fzjjz afa ibuhsz
erwp abn jwx ynmkkj rhgg abn epd atqhs rst rhgg
jtnp cegdsoy gfuvfbg gdmn ahlsc
jgrp diu jrgp onjnml nojmnl vxockc
lakqyuw khq dcpiwt ykwlqua hkq plklx ujbckec hjcvur jnp pvyf
usuvoo jkih ylafyy yhio jureyj
uazisdf cnwlfnf ewodatr woaddkd wbla qmn atdrowe
bnyepaa ntqh xppe ydtsw ppex
yewjwsp jxylmtk ijese ewry ijese kbja nfml zeuwcsh juimz
qbvmf nca zsfreo uurgaiz twe fbqmv ncwi etdcsk atowfp
jeotslx kgdpzp wxlcww pdd dcn ddp
macllv ldl kyluine lbt hbxbr wxcaspp ezwvc qxkeu
ivg gxv zsf ucr uff yrz
tdlwbny bqlrlz tbynwdl lwtbdny
tnekq pdaievs ttwpfh xfm fcaa
zqqhl zbf fbz uqrv bfz ffwavhk foccg
vcw ebqdd cwv eddbq nrmq
hpiusz sizphu xzq sgyehk wgagkv hsygek
vagkxa iou frqdnnr ipcg uxvh vvh eskf katgpiq aqktigp gzvseyi
xkwgd kzfxk pgdy fmtvq ngf rshx zti pamviob ely knz
hwo rteohu qzwoe rotuhe wzb
bsqgg tid dti gtats dit
sjtux djwxv dljwjq xwvjd xnqfvx veqdrtl uxtsj nnkjn wnhilaf unirrp
fruuqjk gtote gooklg bzwhim zfnccmm ezipnf cxwdxa wfu fdca
zcyxb byzxc cxbyz pgcqco ivlxz
wrjh zfdinsf ihw xwosiah hdg xpiabno bilyy azdeczg javuwa
rinlv dcpt qhencba mmb njxw gadc
qwcpua qzyzt cxjsgh kumh byiimas qhsgf qytzz rqqruwp ismyiba xydcxz rwkscqa
xbzefi hltca ibzxfe fkx xizbfe wvaynts
oyuce vzk ouxvj gfh efgbv ubc nyb bxnbhd mtwboe whksy ovmrt
ljrebp tacn bpjler utphw wmfw rcnha
drdnic eyodes rcnidd yseeod
umxmsf kfroz ukhjin awpnnnu ooyyohh tuv rafano jze
bakz lfzpjyg gfkqcgn kzh zwpvk gqfngck
jpaony ojpnya hmro xaaz tovary aaxz iel pbg
swvbgc bbhjp yvrcddd rhj clfu eao afrkegn qvvb yvcx nxjmdo rcvtx
conbjy jeqtri wvujt jeqtri rkhllgw tsdt zowreo qxr qbpragn kuzmplw wvujt
jrpxyp hchljy rkowqb eeaf ltllebb gtksrwx iazx vnsfmc zzrxw hlcjyh
piehb cjdzt eqn kuje rls oaewoz lrqwt lcrrq
hdjowxv uknhlv hluknv pokxg
txiqxfr fyyp pyyf xfxtrqi tvm rtvby cfx trx nwrf kqrxtat alwot
wdaadr stexpow ardawd uejqxc
wwgwjel wwgwjel mtjt wwgwjel
mczx uua lgceb dqru vkcea tcet ruz
jkt yroojr qdrtdu wze ovwz fdmqnr xxsyfd kchytwl hctlkwy gyd
eif irnrce iamhxgh bmis uxye azrwdi sznv yuowb vdlqqxu
dxdjyj hngqwzs yhwku qhsctfe rhbc rchb tqhcfse
fxyxnzs qtxevin rvtxtc iqnxtve
zgbpk mwzxx bgpkz wkpkn
rjiym iub lcyw agbtlb bzhx osv rbtf
emmyu uoflio tinih skpqaj rbor gezbhhv ine mij qlqte uuj ycns
owmwc uhxv pyho ftjh jzsg blqn bszyo bob trbycy mkru
mwgz bbqsmpp fgzs bihhg bbn pjxxexs qrqmt htsxfwo qltqp vqqaxi
lpr wcvy sxjqq ltd rftdgv pdnog ymu
qhcos shuy icdhucu lrikh rwslv yxbgibl rcomhn wakirz
civdmee owlzocl vedecim rogmjnn pix pohcmk dsjm yworm
vzdpxp lvt inufv yofqt omm qfoty qrlseqy amkt kjcvg vgkjc
huhq quhh levzsws sjuun ofgqr cjhp nfxbbft rnt wtbd tbzab
tjftkx xpfcv hvftvhw lpypbjg batrn fhwhtvv uthl arbtn brb sthv
ogr uyuxdco bpjgir edztxv sxtgu jzfmx ihnauz zwegqkr kvkw
mhxthf pervvn gshy jig ezjteq ckkcpy gww
tiljyki rpe prcojy tjkylii moxu
pjsdqc lgqydfd lohck emrtejw axwmo wuuv rfi qzyncmw gjijdfb bljfd xrs
ywjab gynzi relf kziy xmsby izyk ocwoho kqnyh bwayj
bhjlz uonz jhmzuq eiajoos zjnbj tomj bmyv hjlbz fgw jjbnz
kszz xzw xzw prtznyb
ghzk vxhwt thxwv slwpayp qxegmi dawdwo kgzh
ibpcvuf wnuwxu sbf jsj bfjynl cdp jbylnf
epaxr vfhf hvff azepadz pwf sbo pgfzya hslyo rqqj rmklw hwtta
yyolko pwbvxvg xdwl yfje hftep kzzsr kho jeyf yvslxpw kfyv
xmk juyjxy eqno mdwklum reg dgn cirh wmxfyj bnxlgo dlobk
oyv gshqyot jgcqe dsf gyohqst gqgeojo egoogjq dmqpyp
sypianq yss lmhu ulmh itilh ndkda lhiit
qbxxl bxxql ikald nfap qixwbqq
jtqhqty ljysnl nwoj toa bmmyj pal
ahktew sxody nkvsf pbxyt baws wgwfwej bevgzm jus hcvajfy kzrb jwgwewf
jzsb szbj ujngwf nfuuf lfiuxdu uufnf orsy
vgo hto isstyul gau wsmxoqw
uxw itwf epaw hec wape hemol rpwyosc xzxmrll eetz zui kagca
mjncux muv rygdeis rygdeis
qgkqjvf iprzibd fkvqqgj llcrl vbh vlf lllrc zwrunt
dslsa wvoex eqbwj tjem gbx ayn xcan fnacl xggxon gnwjlh
yzosv hcxjiz yvon gcgd
bixpny ecln sda eymt bjiwk
rlcad lrdca adoqfzs rgty mds pwb kmwj
wkai pmryffq rrdmodc wgyx taz yxwg nkap
auynzwc vzg uapdv qkrh
ldmuysp oyu kpn ejbl mfifa bzs hwyn brlw qpzqx uyilao ysdumpl
czoxoj pwnultl wezolbw lyk aonesgb
nqy nhb nle yycp lgtbo ojf dytwyh ufa
rwr eph obg peh pejret prjtee ovgz
vdqf vdqf ycjrg ovzl lelbe vdqf
gvagdqm gvdgqam dmb zaxe nepzwn
emwh bkkbgec qwdgk mhvfsrf wmdfpp ekzuua
mbqw lgkyazt ckyhvnq uladwo owt
qwiwd pbo tkjoqda zapo dygqopv zzdlwfn
qty dhb iinncba ytq kvh idgoevt chx waq
ulffsvk vplsz ulffsvk uxsh cpwgxd ikgcacx nrirke uowcjvn
gknmxr grkxnm fco dilyyj grmxkn
saqxkh uhue nvu fef xsuxq ekyyoc bcaavd
qltwqa vrmpv vhra nof yprauc vkreojm eaq igiy mec
wvheiyg uthy gpvcs nhnjrne mqaejr tfnsly zfbhn entcc nystfl cpq
zxv jzk dwsjgrd gqqxhp xqxu naunwc yeh qzpkz awcnnu aoosa icadax
vpmqmg qmvpgm tqs mvpqmg
inehzu zwxeoy jxia fcyzxc hwikd
bzwnp kamsen ajpn kdls bzh xqcb bzwnp cmjnfa wmgx
hbuhc qgvhxy smzkxh zzebox hbcuh net wyrdppc yvgxqh
oeum oemu iyags xaipdi euom
tqljgoq ghtdhw xhnni lux qltojqg lki zxztda pcqjif acpzvwy
ydijaq kbyjxpu onyd hsfgz geqvbg
rwoih xog dtbzyr ryzbdt tdbyzr
vcdxf zosw pardxfz bmb mscmain lwfc jvq hbszcqh fxomsmm ahnugx
zutsemg pqzil ddv nsstz gmeuzst bedvy xkzzjpw xlqbd
xxf ltnnu yeb hbml agj meovtjr qrul kexerkw xxf
tqrpd hhcx bmdv nlmr pnu pajdtc rpatqi yekedx oeiuew epsshog
ttbfpv plairk toh jagfsg njnqpa tmwh vwqp irtxv
vdky uwc tkkkztp vdky vdky qlcw lza
rzie yundymy pwgx wtwtbg kpiw mewnb liveysj uvsbn
jgfvyny hacg pzra arpz uowswu puzsfu hoe heo vrq naup
hqv vrl uko qgpikho lligvxa wdld qgpikho
whvby yomxwj dieffc jkprinh dsaqy yfrnba woyq yexeb mjn cbszn xeswvvo
wowtgu rciyg rlas bra quyfec ihe thuu asxhscu bsbdpbi ogxosu
vydsaet tvnkjq piedkzj foeiqz zqivt iatsju tjnqvk drauaf vqitz invoz
cppn jqzw zmxr qksuas iifmjg xtkgf cppn cppn jpsd
nkifpsq cxdx bokxhm ebww kghagrp bofhrl grc cheuzyj
ibgrlvm hrcx jjuoh ipmt
hcoqkh fzt rgravb cimauj jxjq blct qhc vjxw pqpg qzp
jycxz xcv czxjy vxc
liljaur cgmg neldxb xfummcq yfhiukd dnqhl iolxn cmewhb
hpvoihj fkwokod txy uuktw vmqqb dpldzh yxmcay cyaxmy xycaym wekr
ccnaf wuxc ecadb vbgpt ccntf sezo skjdkbf fnctc
hqdtwho kdhyman bjtcjvr bwllva ncyffyr
xprn jrrvmj pdw yvexm ewbflbe eapml rvrmjj xmevy rxyzhf
wjcbpy qdgtcp cfjh muww fhg sgfdleo nelpte yucqa aavev
rci vqypsqt xmg rzii
gramh wwprtc ampdhw dajr
ovrm mdyhpbl mdylbph aykz
cbmo fxs nuugu guunu upt ljjuhjw nituh utp kxqc
rhabal rhabal rhabal vah lfrs
nrq qway ftzp rtjcks mbygdtd hsiqbh wypqb rtjcks cllp hsiqbh
ywa anhcf nvd puqkwg molrwck wsctx xvd molrwck
wox jzq jfen wcvus cswvu oxw irg lmu tpj viahm jesic
qenad neqad smlgi ydwzq ppdemvs ucyuf qtunm eoqx jlgv
sucpl nrdwbl ltvetok npbw ozzw hafyay sjmui sjmui jkqlq pyn pbuopx
nxgaiu ybyl meo kgh saqjaz xhbqr otelcyp vkwc
iqrl ldjlwvl ajhrl dnhutr gkknyqs mcvluet fgyu ogiz cxo aiunl orb
psd cyq xpoyqny yqc kozqh vonfd uhozwz pds hcpw
tvaxder tulwmw qiw avddbmh irog vynjzcc refx efxr emnvk
myjx npqk whm egw kpy igrrohg ukglx ldnuqw caqg ynx fckhnsh
dafv bkdoqg zcqvbco xgikoac cvbqczo
rtzhpwk ukuyp bayhzp upkuy ahbpyz
oarcuv pnlkxvw fqdkj hwzsz nauwl lpufibz vzfbgc unkluxy rwh xuknuyl
vxhsaj ppdxw qrswqtu ulwv uqtqwsr ppxwd
cww cww cww scu
wiiikwa bfpewt zbgxfkl iqpk tpbwfe aazdcxj ipqk icggn fwn fjr
net ovxuwpz yvzmzd yvzmzd
xgar czuhp vuhisaq fgrqxy evvrtf mnmar lsk
hld mxuedug itswju vmmejqx snzslqj toe bbmugph mgubhpb mowj nrjnzu
qbz ouhye hsldmp lcf hyhlrb ewvle zko
cke mupaq quapm eck
owu zdt lales tzd apjjo fhpx bmuktbw dvehpz
libvl zxypk azazc vtsom ohdzycb
kiowxnc scxygrf ckxnwio ycxsrgf
vcjj fqz lfawfx mps zhv qykch vhz psu zud spu fnpvkx
scfvum fuktgk tua ieosetl wwmjtt exnsw wwmttj plvd pfb kku pdbom
wkfw snukd wkfw gyaojdf bjw htagy cdsp
beh gatqxcu ibrooxr ssww orrioxb eenkqz
jlv affah mtbemf tylh aafhf
zqfajd uwzrw csouuip qzadjf
gsnlrw tcel hha tfbzrp ild aenqa
iirfxef kdux yvj vbzgj
ibx pfll rgkp nancij llpf xib gbkfy
uvw kkbavj pznsnk okigtxl ogitxkl eobbs xhaz wroabn ltogxki
bivdf lotvmoh vrb kpaeeue tdab qhukcb qmy kuqf kesu
egs hbsfeu esg twxko uib
ocraimu qilp ijmx eco nhevqp juxf ksejr bcqqau uhpt
pyx jmpglf juokd dxszjw cml vcjge pfg
gxwrt btmimse dkpbha idmz mtignka ngakmti
dpjhm jyalra hukf imocr lkgt rqywn quhe fukh
nbau xyc bdh yni xaawxm cyx xwaaxm akx gyodqe htbifc
bywdxe bfrp rvb rndl onal jghiwb nuta aint qlciwcx
fpic yrqce land soxhci qzc zoebsq hcdohcc fzhcl iyxb dqinum hchdcoc
zok ghgp zok lmk
ozfz zofz dkdekzb sqc
gfti zuqvg cexmtyl qwuqnj stepb erduqhy cuoizcs qudyreh kqvfdd guzqv
jrugz jzugr lmqu jgihgo hjfbz duxkn unxkd
ckiys dbqmi ckiys ckiys
iylp uvvdp pluifaa djo
esxec rwvel djxppqf jymwt ilm aiz upn aiz wrfefwi rwvel
nitgjr pokxuy puhdwg qtxpb veylp zqvzkbd lrvpcgu zuy rnigjt ibci
jboyzq ogcldr hlon ywav jqqtz qjzqt vyaw cok
aqdw jxn hqknh azbylg
jya qpxtmsj hqrtsgg qjtpxsm
pofcs sxw dlvru dlvur swx
yphvvb qqyyfsp sjkbff dqyerxe jxzes oof
pwbya txk bbwsj ywgimd kmdpc bawpy lbnt
bkbazff ldmaq tyfl acqurpy ndnrp
asw ctiv mnxzyc weeuwb gsn bzk irbyhxl cgqomj izy zbk
yrxcrbt bcrryxt pofe wwzl
vuaqez kbtuyai vuaqez dxqud uvo gmhtg dxqud
tpzs gqdxpxo zzpgta uurjx xpqxodg
cil lsv vznqw vro zqzvjhm jhgauzw uxnwk lci zpgpu frjvyzo tsv
zfvcuim gwn gnw dxfppok
btb goof iwadca aac tbb jha uvzi
qah ned ipmure kyta ffhrwe njz paq kaag xmlui
rkmw vrblwyy gpax hxsf zpbza gypuwf jbib ypcjwd vrlybyw
yfjljn uxpvg huik jsnah nkhsg yfjljn lqzsz
hagjlqx agnax jqalxgh rvjgtc mjrmph azznzcq gxajlqh
ipki bhoabp rmiyl dmjyxl zzsmap aju
tyjrr rigrf ciq qic avmwu jtr wpq
vuf cosgytm toycgms ufv qzpcbrs
epzgxr lydrsj ezxrpg expzgr
ecm prj kmak makk jpr
ccwyq txy okj matxa socoa
zrjphq gigayv ywkfmru yrwukmf fxjjrha gqkxx zhjy tisutx kufrywm izjfj igg
lfhgsro gsroflh wrpo lofhgsr
kgkgj wkhnab ubrjaoa ubrjaoa ubrjaoa ggdgh
hztutpn epnqmz ffcroq mnqpez niibpn kdloak xjui ozttj lyzsc pzgq inpnib
kruz sjqp mmd hhdxjgc mauouma asevvo upjwqi hxcgjhd etqzagp
zylf qime cho oraid svytv gqrjufv mker cho vnkyiin tjms
dotjul qyv hnh cibtg gdpauyx wzp
fabtira ejxoeor cqyethv ndjrq hnxn joq otng lrr csytrub
txhgepd fwdaanm nawdamf pxine qqrn pronw exnip qwkimt rvy
kuxzhi jln urzxtw rzu ebsuylm tscru qwlhfgq nnu nuchvz vuht
cqgu camlr umkltcf stx izp rtdwxff wkfvs
jhje cxix lefcrsu nebv idfzhic xqri xkft
utzxb znb ietupd uqgbhje aobip oawjwm hetyan uqtqv hpwzyri kwxyu
jvzvbt xuyvp aegdkb srbw bzabpf lyfriez cruyfu
nhi nih aeb ihn
hcf zypt djcm pkjx pvhh
rhvxcfk exydvk ids hybme hnk yfchvs mjbo meocn
rpboxr rxoprb hdzje zhedj
ziildbo apzvatr vsv isndq ebxyy ntm tdttg wkvdh qnids vkdhw xxolip
ywu uyw ipcjz pjzci xjn kvgk vsocprw
euzo njlpv ndrlhi drlnhi ivmjkb fjrtxta skvgmrd
gbyvj dkck gevpfvb lhadhx rgjcdn yraxh bdk oen vqryd bkr
vgkp hncttxb wgxh gdyjo bbdfzvc xhgw rznzgda yxrrlo gxhw
ifjlb fpecyic svhjp ilmj oxgr svhaf
vbqky lhccj xtmm xzjyykn oqmdq qywir bswly
euxxziv totzer vsxfx leo djho uoeaz edaig fbu lumbi
ooqtwq pvo kid vpo jxin bod btqc fbyuz
jhabi mronu htqqyz umjcbv sgnbp wyn cetmt pcjf
tnrkcyl dduuhxh rylkctn pwj rtynkcl mzzfomr
rxx ldqffi ulappk nltawbn tplhb kyb cqyi
vzkw gviooah vxh xeae ohvcad oaiwcj dkx
sdofdjt hcifv dqws sia mlwm vfich kavh myzue roops mzuye
uxs nlbmjp nlbmjp tlaxa tlaxa
ynnisp twx xtw jgkc yinpns
kumorsm wav xhx bpvz clqc ffmadzl ndny ymslo lobv
ljzabj tqhves mezh pwn wue dwfqq lynvtt boeknvi xqbd pkud tzlanis
lgq qiikzl oihnsr pivtjmu qhic yvmeebg rxu qgl yuxnqse dvu faxqez
ldk mlwja vmdqr yzlxiua amlubt ejmzfx nonm zhkxbn gaqbnqq
ttc ctt kneknx smtnaft abljip tct
uybhbiw zwojzlm cfxoopp abulenj znz zzn opllzmm yufk witwxzp
qvkybwi rdbxb qiuizmo fqgne jgot jxz dqhapn
vzinf ehaley amnk laheye invfz
pedakl ivld agzyhr wmzba tzzzg bazwm wjwgux thrnxkn
cmyhae nwfs nfsw kmh pxkaffq
vdf szupev tyunp qiiu deevxmy wozvtt nelnr kgdexy gparqj hajavz biizn
pwspk skpwp ontbjee pkspw cfbj
ihsmh djxtak wkzllao oyr djxtak prc
uhvihqq jrgf hdfek pdrfpt tghz gthz awae wcygi wujti svq fhedk
gnfhsj odqlt netmsul rviio nkzw nkzw
xyvc clxw cyxv lxcw
duegck pkviu npwsp zdx wpvn dmxgnv ixv fybs xteru
vih kgk hads boaddu daiwo hozoufv nef vtcplc isiw
tzqoo dqlgvno jzlay sywx ecej addt ecej addt mnfcu
ymgmby zegudpx ipsjai ger wcwjw brzebb
eqekxlx itra xekelxq exqkexl
rciu ojaa ircu nxjga puvmwou remgu
sltth pprimb slnxopq avtir hvpv ppww fhfap wisn kzs jcuuuuf
xbppc ydpbq zhjh oym iljzvk vsb
ueye shtps uccehi ccheiu dqm yeeu
gwywf lcpv qza qza gzuovj jfzffyh oybfxqv
aawi ynsvdco azdoz cqr tnyquq xlyvbx eca kcalpes
zumgzhy rou kguqa vubw bwgd qprxcg etnbev nqmi
fyd tuoz uwclqn cgl lrpkf irz dizv nxze clg jghx jbpt
kwuanos eorjr tcahp kwuanos cyrpfji zxayggd kwuanos jkqt qqvbork lizk
vtu ovje vhg ovje vtu zcy hrhtr puawfgv
bliz exp wot svxv epx
jiqgxwj yips hjsatc jgsrno msfp vxvbt bba bqmw xjgpgog
vpvypp ggwp wggp gndp hedpse afji hcqgof
hxueubt hiynoa qqzaj ohb qway
akq nfnes sdrlza nfnes weq
udxpdpx gctuv llhxuow rqtetm hdbnpte oebapv civy oeobu ftgivd pykj
pbgbvn jgmr xrz dfn gosjobw ndf
gnf dtbsnc fwcmml tscdnb fgn qgadusl eifpk
vmnv yuxrup qcphi tanc tnca kjrv cphqi
hclggs sghglc fgplp odn pfglp emkrulf whwtmbs qnuyg
wcxtr ani ain sha hsa zxbkf bzxokat qezo ljqxi xqcwfmd dxo
waiq smpbu dbyka uibxjrg nze wiqa rfpts ddjsjv jqqjez bpusm
lpcxf vsbj owjwc tuqj vkrgrh jsjdepv oil lxrjox frsxsi clr
vzunp prwk nnd rfs vpuzn
pqpqv lvsk sqxf nhobsm hakbn ywj
xxu uxx szqnmi lnwtmx
akq nmlw fupwsth jduvhva
nac wwlxqck hpbce vxxqa fyp xvxqa kxwclqw yvlmv bfwi
pzxjbj nvwv mdooiez vvftp enjrsck iypu uhru fpx omtd
llxgp qwf pwaj cuhb scloot hbcu jgp vjw ooclst
sisd akawvzd wvdzkaa gyoij ikt eeeosb jiwiup
tche vxj sbctqv jvx gosur usgor ibo yqxo qqgd zspl
cidd welisl fxblxqk qxbklfx fbdoqcz glhq iylodvz zvds ghlq
cnsa hrxst mrnkqtj bptq jmi cpbcofs kveyeur uzmga modphm rtx kntqjrm
dvyup usfaq rtghoec bvcos fqsua zohwwg
onf vncybi dlaxni oqyqqkn
okfwa qyyx ebnv llql nphq etdt ytgivlo jwgwz kiob
ann vqnqvpx wth lpwid bjvzw xpwqxcj azg ioeyzzp onwf
smy epzomx xep yid zctvrfj astdj cfg fgc eriuxt
rljqgin wzobzrh cuwtx vcsbx tmg tuysq vxipgho
ewp rsrnsj wgeyin lrji ddgt utol xxwut fjiwopa
upu ftvqbk tfkvbq fdwga rmu puu hbiasjw
cfl lmqkb lfc wbtlfi uqsjs ejgmphi tbliwf nzcela gzb
zop unwmiu acull mkwh hvruknw rfk mmhaz iqmenq fifino
iczua bjut tlgf zicau jtbu
mtka ipd mdifj kps
irqkysw xfsjl tedx yckkbx iktxb sqxn pbfvubv uudzppz
mdrn cihat wcext kufs awwtjok pfjg
wdevt tyo zzbp pqlqq wdevt
yhatqkv ntuhw tdfd buxazh xbcsv bas gkv rbzi tddf jbj bsa
malip hiiy qezz yhii wlfojre
zqnfll bssveq lprwbep bhqml tztbt
npnxotu yupdytb jptqo klfydfe fpucmfq svxcqr unopxnt
gdpz gwj iytiohu efk ctjzf asade abhotq brmhu tbtdur zzksbh
kxft klzslf tjdzciy lzslkf
ejei ezmemvg xlt zte tbwhz dgnfpao zotck wus uaz gbwbb
dgednf vypmbs eiytot empfmny
uopmui uehue wdvzt adpfcif mutl ifaztka vydi xumtz orstno
dleero olxiq gxnlfm nfmxlg wloeavr olhrwg hrjd yicj ymyeex qav gxyjgfq
hevj rqcne zycgb qgqtn rqcne ptfvu yyyu zlm hevj
zrkhuh sttnkt hkuzhr vqtu
ppsfm kcao qjq dgadglx cxaawjn pbucfu fed qgioarc dfe ricoaqg
vmawf oktunea zraoir gkt zraoir jcvkqoq
mqgml ecawug ugwace szwul iwbmooj owmiojb
auggaw cypcuw npci vuyxijd pofswjx vdkrgx xylk rom ksj
qmwx jgsrdj ikva xzxw avik
zzhcqu rbg pywjdn wyndpj zchuqz
wzd wqycftu yldezp zovuy oydia hovewe
kfid qkkk thak qhbf rvzlzvu uuxh pbj hkat gow oeqcw knqqzha
sua itv hfpg bdqye bznlrk hfpg bdqye kvir kaai ggtz jqn
ulggl guitamm tkpckso fupacz otxtqpd jxnqc
ueesb ndyik vjftz jgqqv nrcf
krh dqpmsw fybzynl zhjbvkw exefc rhs neq ldprb bhhvxm pjwirun
ymavl qwxr yavml wagwc ekokrpq zewppw iumcgin cxdvwx
wwdukav kuawvwd kowv dkwvuwa
eazot bil tzu vdwwbm fvauwrq
esq tixokph yspf ztoxfut lgzush pwv swh pwv auqhuu tixokph
pdbeyxi poio mugfkb brwbbx aao uszw fokjeb uswz
sbs ryjr ptispi tvnhu htunv vthnu
czjmg hbdjhvi jrkoy fpgwc syafy aar kvnq eaecsb wqzpx
twtp dvl uvyje qtlzj dsvyr qpjnj eyoigx bhgpccy gwn dtuf
mxit xunctu vbyks wmqc jriuupl ybvks uncutx nsoxwrb ykt prc
yye mgf uhc irowpc dsdv iwaxod ftavlj dxzp tcch tcch mefz
rxe xwrrgl xwrrgl duu rxe xbbgoe
ucsz akswcd ojrmqq cox hgfh lxwu ltnnf cenikcp
opjhdp svwezr svwezr opjhdp
qojlkl ircxqnt utfmdg fcvr vehkcvt ufmzcpv xwlh ddavv xel bwlz fii
rzkayeh iursm zhily hdnq fqydfvt uwoy hptpiqu tdqy bgr xdr
ymruz umzry hbltwya jhwhzk flh tahylbw bdbaimb qscbp ntkuf
uxpato owsqyao vaog oenomkc usrmnc epua vzkppls
qxqczbk qyguz alawj xgjawtw wxtjgwa snfcdmz
fjfgos rmpd mgs vbk dlls jkljao eoovdfb ucdvaoq qmjmqku ney porr
nmcrqz zcoxpk dlnzksd ymh zyg spxss ruyk bychq gsgv eusiuid mnrqcz
jbzadnx lzl sdamer okoico frqisrm lxet agriw
xceoqr qai vahc jjzifsn exg
igjpn wfy ukn aag quro wklsq cjq bgtjrdz gmub wyhh
fzlwnm mygfn vkzwvw zvhsex gfki
ijvzgai ebmeq wssfmbq uguh sfuutm nwkgmex dxael liakdxs rnf sky yowpxc
bjzkyjh fced nji esowk qxsubsk qgtts
nkdgo bbjfq fgnxnhd gfjchl jetdb xubsgj eiju ldlm oxsx znft bbqfj
xovcnob pxfe pmstes yzkdm iqlvha nmcziix fexp ivqalh rxecqps
xpyew xudfud wwqe qhfjlcu epv fnrbgyv ihli qngtx yjlfg ozqbzn esp
timl gcohx vqzic gzm shwlkkv icqzv urchuc
xpqq gaqzwo cci dowahsr gaqzwo
jjsagdl umbpxre kyre zvaryft tmw pxpnjy
aqovcz nunq nnuq xjrvvh autjmit jiatumt
elg lps lge zjjot hwz tmqrup xaxxmo zlbzp uftd fukdad kvpymsm
iokwzal ywti zbdmzbu lprywe wbgbwza ypogbga kzliwao wstqi eqm keaeaj gbabwwz
lwfpk mhufe eddzgd ljxyqy vhzkct uemhf
lwqil fzugdo faq feppo usl llwqi
nje hthr ropq qvcepu bexszfj avmzjvv zajmvvv fhcd xnc cnx qnuaux
kvksn dphbyz nsx wrcc ccrw
nzpa pzzunfv ygzjy gxrrtcj hrt trh pwxpg yifgjmo fnupzzv wbzx
aepti rbojui ypvhe ubojri tcema aan dntkw qjx bfvmyos tcm hvoqytn
qpwq exu jvsiwj gsw avr vbemldy
xsbzpf xbzyvx xax sxh vpxt gccy xxa zhgbwoa hwwxoky fhvdxfc pvtx
pnsa ovtjolz tyutl eyjjzt jvtoolz owbypvr tytlu ewtzgec
cyg dwwk eihsp aeuk bbnay aluwyz hdmv uaek mwt ihpse wjhnkeg
fhzx vjetz vjub tejvz
ewwyb jidhu pyvyenn igtnyd tiwr akwkkbi myz xxjwb jjrdeg
jbkuw kwir rkiw ubwkj
bltffuw lftwufb hhsh wfbtulf nrxaa rlszi toijxnz czlci
bqrm pga zgblgcw pgwhhn lcgzwbg bcgzlgw yqb
mhjj vjoa gnjlc kclcr ito ofksy giavy fpqeioj
bkiqmif izidbui sttxxi bswhkxp sduuw
mjgnvw mjgwnv ojzyuv gvj
qxn kkhc whd fgwk auzugg augzgu kqfov wfgk
spdxbnu xpfofsb bpfsoxf ahjywql spbxoff
bwqxhlm wbqlxmh kqgpl fyzgf guhkvgx ovk qhmp gnrmu wvd wedj
vvwf hcnc vvwsngj qedzoxm hcnc qedzoxm kjthdi cbwqep qtvu
gio iqklmro noqablo bab jiqc rwebyg rqkloim wzmgs uunl amqs iwj
snxj szobqt zcgvwv wiyqknu
uto jteikwd cew gqsks hmvjtcy sach
zpgl qnkoex amhufmr figns upv xezrl rjleak nwrna
pzkvrdz dtonazj gtr gfxucuf lstjl lsjtl rgkope kzpdzrv lyptn zfxjys ttk
ddxgm lumlgki jhv doft kok swy ckds swy ddxgm lbfbdv
qfs rcufzgz iaiqw qfs qfs
nvkbo sgv mquwb ritpye nbkov poex hraorm qrrr qdt qefl
irxannd fiud ehyb ggx plqg pvvn uuptop tcvbm abuf bcfnmw
qwya ukblz epmbfr vmlon yqwa
hlo mmv vmm mvm
svzpxun yugbbe sbbpxs dmy xspbbs zhpovyf fyovhzp cpbt pke
zgk gft zybs zrgcoo ypu bue htgo
xnesq srsx pkzaoh cfqzugh
lntd nvxetbv clykjpd svmibpx evxtvnb yldkpjc
jsqq tzwak hephg eqwczd ioisa yim tmdifn mceip
kuwqz wzkqu zwchmj lfec uexne iztp llityt
kvamkpc pvbryqh ion cwizjde gln kcpvmak pzzlw gnl
ydeqf bfaab sydqhbp smsxdjr pynrs cqymt
onb eiab bno nob
mqslq scnelxv hyllrf scnelxv mqslq wmnbk
pttu kubby lgop bbyuk gsk skg ikktlbb inbyvz
xznvl zwtdj vbxdyd clhw
hgy zudelp ickc drfjgn iyws xhc
zzv wik iorhat qkb kjb lykdz vrce yjsjwj
gyw xzgbi efus uuy
hwcy ujdun bjjuvd jbdvju onnk xeyy mmp onkn qyzl
jwfm ptjwrbl hhuv uolz adyweh qpj wxyogp igvnojq jmfw pqs fsnirby
        """
        "2 1 1 2 0 -4 0 -4 0 0 2 -9 -6 -2 -11 -5 -6 -9 -6 -17 -9 -19 -9 -10 -13 -12 -21 -21 2 -12 -22 -20 -12 -21 -13 -28 -9 -20 -26 -21 -28 -3 -21 -11 -10 -32 -15 -42 0 -31 -26 -45 2 -49 -48 -38 -52 -28 -8 -29 -30 -1 -17 -58 -1 -16 -27 -21 -34 0 -7 -45 -13 -5 -28 -11 -74 -75 -10 -50 -20 -31 -19 -78 -80 -42 -24 -50 -65 -15 -29 -34 -3 -67 -36 -69 -95 -59 -69 -3 -92 -92 0 -78 1 -22 -89 -25 -42 -99 -65 0 -51 -67 -38 -103 -8 -69 -64 -54 -27 -29 -47 0 -123 -53 -25 -62 2 -38 -18 0 -94 -85 -19 -19 -76 -25 -49 -111 -56 -103 -79 -124 -107 -38 -14 -69 -32 -1 -113 -109 -20 -73 -15 -101 -132 0 -72 -28 -91 -89 0 -130 -32 -148 -28 -166 -28 -15 -57 -143 -16 -88 -158 -172 -126 -21 -148 1 -10 -2 -24 -174 -108 -2 -150 -79 -1 -72 -108 -126 -158 -153 -125 -47 -107 -79 -168 -93 -160 -140 -200 -131 -188 -103 -20 -122 -153 -170 -122 -144 -153 -176 -168 -76 -163 2 -112 -126 -7 -74 -128 -144 -20 -220 -102 -177 -3 -172 -26 -3 -78 -179 -120 -20 -16 -175 -78 -84 -153 -154 -23 -29 -161 -222 -202 -31 -169 -244 -242 -56 -32 -62 -81 -6 -178 -25 -121 -122 -99 -110 -86 -23 -255 -6 -235 -63 -1 -80 -161 -71 -93 -75 -16 -139 -101 -94 -136 -21 -254 -214 -258 -56 -101 -103 -124 -250 -58 -136 -62 -192 -232 -261 0 -21 -255 -51 -175 -222 -63 -290 -134 -19 -243 -272 -112 -279 -110 -261 -192 -44 -90 -307 -106 -118 -4 -27 -175 -178 -309 -11 -86 -124 -250 -299 -284 -264 -11 -136 -29 -174 -3 -163 -211 -226 -271 -63 -283 -98 -335 -299 2 -104 -219 0 -329 -33 -248 -268 -121 -294 -41 -206 -3 -115 -256 -326 -168 -62 -221 -51 -113 -113 -286 -105 -126 -8 -88 -1 -344 -266 -201 -175 -52 -109 -192 -272 -197 -45 -218 -181 -329 -355 -330 1 -57 -226 -200 -213 -387 -148 -28 -65 -283 -145 -37 -299 -189 -387 -46 -290 -358 -51 -89 -52 0 -279 -342 -243 -257 -244 -83 -152 -218 -194 -79 -130 -162 -394 -280 -140 -280 -95 -22 -424 -204 -34 -127 -384 -161 -248 -7 -99 -381 -173 -147 -171 -9 -377 -319 -248 -61 -263 -343 -434 -257 -297 -419 -413 -387 -448 -221 -248 -178 -32 -50 -327 -127 -388 -13 -349 -312 -225 -200 -374 -252 -457 -165 -244 -61 -125 -26 -127 -59 -334 -391 -20 -63 -66 -111 -219 -363 -354 -430 -29 -485 -427 -481 -466 -63 -336 -85 -28 -305 -346 -207 -202 -291 -373 -128 -421 -64 -315 -160 -54 -282 -63 -116 -209 -332 -27 -140 -492 -343 -18 -84 -291 -305 -429 -442 -194 -447 -126 -381 -328 -214 -179 -93 2 -408 -278 -381 -105 -527 -213 -77 -320 -191 -460 -484 -184 -42 -508 -316 -350 -86 -310 -226 -347 -365 -58 -49 -167 -329 -281 -68 -23 -66 -532 -549 -126 -510 -419 -305 -47 -291 -99 -96 -318 -174 -199 -317 -520 -194 -422 -247 -539 -21 -340 -545 -145 -159 -145 -25 -515 -559 -500 -289 -75 -351 -417 -564 -74 -176 -476 -585 -513 -590 -235 -342 -403 -161 -196 -197 -528 -343 -385 -557 -99 -514 -135 -159 -101 -559 -363 -155 -186 -127 -532 -195 -383 -554 -92 -160 -142 -93 -349 -180 -393 -68 -575 -553 -496 -173 -3 -267 -382 -394 -44 -65 -316 -25 -390 -110 -418 -315 -220 -453 -428 -77 -298 -44 -412 -493 -588 -90 -418 -296 -355 -89 -176 -628 -623 -352 -401 -614 -318 -353 -298 -256 -327 -560 -347 -321 -309 -232 -89 -22 -522 -380 -553 -568 -570 -174 -278 -422 -561 -223 -375 -555 -184 -218 -54 -135 -582 -490 -122 -447 -384 -285 -131 -246 -222 -431 -41 -340 -417 0 -186 -346 -682 -670 -616 -245 -453 -282 -565 -442 -135 -224 -516 -534 -208 -173 -413 -295 -180 -595 -524 -207 -500 -102 -704 -705 -519 -118 -523 -174 -596 -115 -596 -124 -530 -78 -526 -72 -730 -330 -722 -116 -273 -19 -293 -745 -269 -591 -660 -445 -79 -267 -182 -428 -347 -448 -137 -171 -350 -48 -385 -641 -573 -690 -729 -310 -25 -555 -101 -112 -578 -102 -778 -374 -292 -270 -128 -159 -219 -703 -250 -554 -778 -311 -417 -645 -38 -397 -21 -117 -571 -27 -267 -443 -93 -166 -395 -631 -582 -434 -21 -322 -537 -624 -472 -631 -34 -810 -251 -558 -779 -445 -519 -486 -245 -332 -758 -507 -264 -706 -744 -334 -714 -266 -478 -726 -747 -782 -386 -508 -471 -154 -111 -401 -595 -790 -501 -266 -66 -257 -20 -432 -22 -409 -228 -671 -767 -655 -368 -640 -746 -714 -323 -678 -137 -613 -309 -566 -755 -164 -461 -825 -569 -282 -775 -564 -215 -576 -770 -103 -553 -335 -153 -616 -707 -243 -336 -632 -399 -491 -612 -864 -609 -6 -215 -860 -570 -605 -88 -81 -161 -818 -806 -793 -270 -808 -754 -302 -486 -433 -734 -576 -762 -72 -859 -719 -645 -172 -853 -756 -234 -58 -177 -159 -756 -205 -894 -904 -614 -117 -642 -782 -200 -853 -2 -335 -36 -759 -102 -716 -902 -917 -101 -902 -810 -384 -883 -366 -20 -328 -409 -97 -806 -560 -386 -324 -581 -582 -203 -43 -757 -289 -867 -835 -449 -72 -677 -261 -249 -658 -502 -130 -575 -213 -433 -77 -866 -749 -267 -220 -239 -705 -825 -494 -980 -902 -152 -843 -921 -332 -880 -136 -310 -245 -482 -521 -773 -987 -220 -96 -924 -997 -645 -944 -546 -665 -254 -664 -631 -587 -91 -115 -597 -422 -65 -772 -557 -113 -899 -676 -17 -540 -160 -824 -67 -469 -493 -347 -535 -768 -618 -790 -943 -352 -601 -869 -623 -975 -368 -842 -794 -827 -1034 -438 -221 -823 -655 -128 -657 -22 -200 -803 -307 -901 -2 -861 -988 -112 -215 0 -210 -261 -1014 -147 -485 -84 -753 -674 -701 -151 -883 -467 -990 -777 -1042 -509 -604 -885 -1022 -524 -736 -23 -255 -581 -837 -342 -852 -58 -666 -455 -498 -92 -1030"
    , Cmd.none
    )



---- UPDATE ----


type Msg
    = UpdateDayOneInput String
    | UpdateDayTwoInput String
    | UpdateDayThreeInput String
    | UpdateDayFourInput String
    | UpdateDayFiveInput String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateDayOneInput input ->
            ( { model | dayOneInput = input }, Cmd.none )

        UpdateDayTwoInput input ->
            ( { model | dayTwoInput = input }, Cmd.none )

        UpdateDayThreeInput input ->
            ( { model | dayThreeInput = input }, Cmd.none )

        UpdateDayFourInput input ->
            ( { model | dayFourInput = input }, Cmd.none )

        UpdateDayFiveInput input ->
            ( { model | dayFiveInput = input }, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    div
        []
        [ dayOneView model
        , dayTwoView model
        , dayThreeView model
        , dayFourView model
        , dayFiveView model
        ]


dayOneView : Model -> Html Msg
dayOneView model =
    cardView
        [ text "Day 1: Inverse Captcha" ]
        [ input
            [ onInput <| UpdateDayOneInput
            , class "form-control"
            , value model.dayOneInput
            ]
            []
        , solutionView "Part 1:" <| DayOne.partOne model.dayOneInput
        , solutionView "Part 2:" <| DayOne.partTwo model.dayOneInput
        ]


dayTwoView : Model -> Html Msg
dayTwoView model =
    cardView
        [ text "Day 2: Inverse Captcha" ]
        [ input
            [ onInput <| UpdateDayTwoInput
            , class "form-control"
            , value model.dayTwoInput
            ]
            []
        , solutionView "Part 1:" <| DayTwo.partOne model.dayTwoInput
        , solutionView "Part 2:" <| DayTwo.partTwo model.dayTwoInput
        ]


dayThreeView : Model -> Html Msg
dayThreeView model =
    cardView
        [ text "Day 3: Spiral Memory" ]
        [ input
            [ onInput <| UpdateDayThreeInput
            , class "form-control"
            , value model.dayThreeInput
            ]
            []
        , solutionView "Part 1:" <| DayThree.partOne model.dayThreeInput
        , solutionView "Part 2:" <| DayThree.partTwo model.dayThreeInput
        ]


dayFourView : Model -> Html Msg
dayFourView model =
    cardView
        [ text "Day 4: High-Entropy Passphrases" ]
        [ input
            [ onInput <| UpdateDayFourInput
            , class "form-control"
            , value model.dayFourInput
            ]
            []
        , solutionView "Part 1:" <| DayFour.partOne model.dayFourInput
        , solutionView "Part 2:" <| DayFour.partTwo model.dayFourInput
        ]


dayFiveView : Model -> Html Msg
dayFiveView model =
    cardView
        [ text "Day 5: A Maze of Twisty Trampolines, All Alike" ]
        [ input
            [ onInput <| UpdateDayFiveInput
            , class "form-control"
            , value model.dayFiveInput
            ]
            []

        --, solutionView "Part 1:" <| DayFive.partOne model.dayFiveInput
        --, solutionView "Part 2:" <| DayFive.partTwo model.dayFiveInput
        ]


solutionView : String -> a -> Html Msg
solutionView title result =
    p
        []
        [ label [] [ text title ]
        , code [] [ text <| toString result ]
        ]


cardView : List (Html Msg) -> List (Html Msg) -> Html Msg
cardView header content =
    div
        [ class "card mb-3" ]
        [ h4 [ class "card-header" ] header
        , div [ class "card-body" ] content
        ]



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        }
