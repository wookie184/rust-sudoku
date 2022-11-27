mod constants;
mod generator;
mod solver;
mod utils;

pub use self::generator::SudokuGenerator as Generator;
pub use self::solver::SudokuSolver as Solver;

#[cfg(test)]
mod test {
    use std::iter::zip;

    pub const TEST_CASES: [&str; 95] = [
        "400000805030000000000700000020000060000080400000010000000603070500200000104000000",
        "520006000000000701300000000000400800600000050000000000041800000000030020008700000",
        "600000803040700000000000000000504070300200000106000000020000050000080600000010000",
        "480300000000000071020000000705000060000200800000000000001076000300000400000050000",
        "000014000030000200070000000000900030601000000000000080200000104000050600000708000",
        "000000520080400000030009000501000600200700000000300000600010000000000704000000030",
        "602050000000003040000000000430008000010000200000000700500270000000000081000600000",
        "052400000000070100000000000000802000300000600090500000106030000000000089700000000",
        "602050000000004030000000000430008000010000200000000700500270000000000081000600000",
        "092300000000080100000000000107040000000000065800000000060502000400000700000900000",
        "600302000050000010000000000702600000000000054300000000080150000000040200000000700",
        "060501090100090053900007000040800070000000508081705030000050200000000000076008000",
        "005000987040050001007000000200048000090100000600200000300600200000009070000000500",
        "306070000000000051800000000010405000700000600000200000020000040000080300000500000",
        "100000308070400000000000000203010000000000095800000000050600070000080200040000000",
        "600302000040000010000000000702600000000000054300000000080150000000040200000000700",
        "000030090000200001050900000000000000102080406080500020075000000401006003000004060",
        "450000030000801000090000000000050090200700000800000000010040000000000702000600800",
        "023700006800060590900000700000040970307096002000000000500470000000002000080000000",
        "008400030000300000900001574790008000000007005140000020009060002050000400000090056",
        "098010000200000060000000000000302050084000000000600000000040809300500000000000100",
        "002470058000000000000001040000020009528090400009000100000000030300007500685002000",
        "400000805030000000000700000020000060000050400000010000000603070500200000109000000",
        "020300000063000005800000001500009030000700000000100008087900260000006070006007004",
        "100000709040007200800000000070010060300000005060040020000000008005300070702000046",
        "400000300000802000000700000000100087340000000600000000500060000000010400082000000",
        "000000071020800000000403000700060050000200300900000000600070000080000400000050000",
        "600302000040000080000000000702600000000000054300000000080150000000080200000000700",
        "047080001000000000000600700600003570000005000010060000280040000090100040000020690",
        "000000801700200000000506000000700050010000300080000000500000020040080000600030000",
        "380600000009000000020030510000005000030010060000400000017050080000000900000007032",
        "000500000000000506970000020004802000250100030080030000000004070013050090020003100",
        "020000000305062009068000300050000000000640802004700900003000001000006000170430000",
        "080040000300000010000000020005000406900100800200000000000309000060000500000200000",
        "008090100060500020000006000030107050000000009004000300050000200070003080200700004",
        "400000508030000000000700000020000060000050800000010000000603070500200000108000000",
        "100000308060400000000000000203010000000000095800000000050600070000080200040000000",
        "100006080064000000000040007000090600070400500500070100050000320300008000400000000",
        "249060003030000200800000005000006000000200000010040820090500700004000001070003000",
        "000800009087300040600700000008500970000000000043007500000003000030001450400002001",
        "000501000090000800060000000401000000000070090000000030800000105000200400000360000",
        "000000801600200000000705000000600020010000300080000000200000070030080000500040000",
        "047600050803000002000009000000805006000100000602400000078000510006000040090004007",
        "000007095000001000860020000020073008500000060003004900305000417240000000000000000",
        "040500000800090030076020000014600000000009007000003600001004050060000003007100200",
        "083400000000070050000000000040108000000000027000300000206050000500000800000000100",
        "009000003000009000700000506006500400000300000028000000300750600600000000000120308",
        "026039000000600001900000700000004009050000200008500000300200900400007620000000004",
        "203080000800700000000000100060507000400000030000100000000000082050000600010000000",
        "600302000010000050000000000702600000000000084300000000080150000000080200000000700",
        "100000900064001070070040000000300000308900500007000020000060709000004010000129030",
        "000000000900000084062300050000600045300010006000900070000100000405002000030800009",
        "020000593800500460940060008002030000060080730700200000000040380070000600000000005",
        "904005000250600100310000008070009000400260000001470000700000002000300806040000090",
        "000520000090003004000000700010000040080045300600010008702000000008000032040080010",
        "530020900024030050009000000000010827000700000000098100000000000006400009102050430",
        "100007860007008010800200009000000002400010000009005000608000000000050900000009304",
        "000050001100000070060000080000004000009010300000596020080062007007000000305070200",
        "047020000800001000030000902000005000600810050000040000070000304000900010400270800",
        "000000940000090005300005070080400100463000000000007080800700000700000028050260000",
        "020000006000041000007800001000000700003700000600412000010074005008050070000003900",
        "100000308060400000000000000203010000000000075800000000070500060000080200040000000",
        "200001090010030700900800020000000850060400000000070003020300060000500000109000205",
        "007008000006020300030000009010050060000010000070900002000000004083004000260000510",
        "000360000850000000904008000000006800000000017009004500010500060400009002000003000",
        "340600000007000000020080570000005000070010020000400000036020010000000900000007082",
        "000000401800200000000607000000800060040000300010000000600000020050010000700030000",
        "040050067000100040000200000100800300000000200060000000000040050300000800200000000",
        "000000040002004001070050090003007000040060000600100800020000100850900060000080003",
        "800700004050000600000000000030970008000043005000020900006000000200060007071008302",
        "080004050000700300000000000010085000600000200000040000302600000000000041700000000",
        "000070080006000500020003061010007002008005340200900000002000000580006030400010000",
        "000000801600200000000705000000600020010000300080000000200000070040080000500030000",
        "020000000000600003074080000000003002080040010600500000000010780500009000000000040",
        "052006800000007020000000600004800900200410000001000008006100380000090006300600109",
        "000010780500009000000000040020000000000600003074080000000003002080040010600500000",
        "100000003060300700070005001210700090007000000008010020000806400009020060000400000",
        "400070100001904605000001000000700002002030000847006000014000806020000300600090000",
        "000000801700200000000506000000700050010000300080000000500000020030080000600040000",
        "963000000100008000000205000040800000010000700000030025700000030009020407000000900",
        "150300000070040200004072000008000000000900108010080790000003800000000000600007423",
        "000000000057240009800009470009003000500900120003010900060000250000560000070000006",
        "000075000010020000040003000500000302000800010000000600000100480200000000700000000",
        "600000703040800000000000000000504080700200000103000000020000050000070900000010000",
        "000060004006030000100400507700000805000800000608000090002090000400003200009700100",
        "032000005800300000904280001000400039000600050000010000020006708000004000095000060",
        "000503000000060700508000016360020000000401000000030005670000208004070000000200500",
        "050307040100000000030000000508030610000800509060010000000040006000692700002000900",
        "005008001800000090000000780000400000640000900000053002060000000001380050000907140",
        "000000000072060100005100082080001300400000000037090010000023800504009000000000790",
        "000658000004000000120000000000009607000300500002080003001900800306000004000047300",
        "020300000006008090830500000000200080709005000000006004000000010001000402200700809",
        "050090000100000600000308000008040009514000000030000200000000004080006007700150060",
        "000002000000070001700300090800700000020890600013006000090050824000008910000000000",
        "300080000000700005100000000000000360002004000070000000000060130045200000000000800",
    ];

    pub const TEST_CASES_CORRECT: [&str; 95] = [
        "417369825632158947958724316825437169791586432346912758289643571573291684164875293",
        "527316489896542731314987562172453896689271354453698217941825673765134928238769145",
        "617459823248736915539128467982564371374291586156873294823647159791385642465912738",
        "487312695593684271126597384735849162914265837268731549851476923379128456642953718",
        "962314857134587269578296413847962531651873942329145786285639174793451628416728395",
        "416837529982465371735129468571298643293746185864351297647913852359682714128574936",
        "682154379951763842374892165437528916816937254295416738568271493729345681143689527",
        "652481937834679152971325864467812593315794628298563471186937245523146789749258316",
        "682153479951764832374892165437528916816947253295316748568271394729435681143689527",
        "792351648543786129681429537157648293924137865836295471368572914419863752275914386",
        "614382579953764812827591436742635198168279354395418627286157943579843261431926785",
        "863521794127496853954387621645839172739142568281765439498653217512974386376218945",
        "135426987846957321927381465213748659598163742674295813351674298482539176769812534",
        "356871294972643851841952736213465987794318625685297413128736549569184372437529168",
        "129576348376428519584391627293815764417263895865749132958632471731984256642157983",
        "615382479943765812827491536752634198168279354394518627286157943579843261431926785",
        "718435692963278541254961378547612839192387456386549127675893214421756983839124765",
        "458276931623891475197534286371452698269783154845169327712948563986315742534627819",
        "123759486874261593965384721216543978357896142498127365532478619641932857789615234",
        "518476239427359618963821574795248361832617945146935827379564182651782493284193756",
        "498716523257839461136425987971382654684157392523694718765241839319578246842963175",
        "132479658847563291956281347413725869528196473769348125271854936394617582685932714",
        "417369825638125947952748316825437169791856432346912758284693571573281694169574283",
        "925371486163498725874562391542689137618753942739124658487915263351246879296837514",
        "123456789649837251857291634274518963398672415561943827416725398985364172732189546",
        "475691328961832745823754196259143687347586219618927534534269871796318452182475963",
        "349526871521897643876413529718369254465281397932745186654178932187932465293654718",
        "618342579943765182527891436752634891861279354394518627286157943179483265435926718",
        "947582361863471952152639784624813579738295416519764823285946137396157248471328695",
        "254379861761248593893516742326791458915824376487653219538167924142985637679432185",
        "385621497179584326426739518762395841534812769891476253917253684243168975658947132",
        "836521947142379586975648321364892715259167438781435269598214673413756892627983154",
        "427593186315862479968174325659328714731649852284751963593287641842916537176435298",
        "781942365324576918659831724815723496936154872247698153578369241162487539493215687",
        "748392165369514728125876943932147856687235419514689372853461297476923581291758634",
        "417369528839125746652748319925837461741956832386412957294683175573291684168574293",
        "124597368369428517587361924293815746416273895875946132958632471631784259742159683",
        "137926485964587231825341967241895673673412598589673142758164329396258714412739856",
        "249865173531974268867132495423786519986251347715349826692518734354627981178493652",
        "351846729287319645694725183168534972725198364943267518516483297832971456479652831",
        "748591326195623847263487519421936758356874291987152634832749165679215483514368972",
        "723469851651238794894715632375691428912874365486523917248356179137982546569147283",
        "947628351863751492125349678734895126589162734612473985478236519256917843391584267",
        "132467895957381246864529731429673158578912364613854972385296417241735689796148523",
        "143587962852496731976321584214675398635819427789243615321764859468952173597138246",
        "783465219421973658965281734347128596198546327652397481216854973534719862879632145",
        "219675843865439721743281596936512487157348962428967135382754619671893254594126378",
        "126739845847625391935481762213864579654973218798512436361248957489157623572396184",
        "273681495891754263546392178169537824485269731327148956734916582958423617612875349",
        "654312879913876452827495136742638591165729384398541627286157943471983265539264718",
        "152738946864291375973645281216357498348912567597486123421863759639574812785129634",
        "174589362953261784862347951219673845387415296546928173628194537495732618731856429",
        "126478593837592461945361278412937856569184732783256914251649387374815629698723145",
        "964815237258637149317924658872159364495263781631478925783596412529341876146782593",
        "476529183895173624321864795517398246289645371634712958752431869168957432943286517",
        "538127946624839751719645382965314827381762594247598163493281675856473219172956438",
        "124597863937648215856231749513786492482913657769425138698374521341852976275169384",
        "872459631154683972963721485216834759549217368738596124481362597627945813395178246",
        "947326581852491673136587942284735169693812457715649238579168324328954716461273895",
        "215876943678394215349125876587432169463981752192657384826743591734519628951268437",
        "124397856835641297967825341241538769583769412679412538312974685498256173756183924",
        "125976348369428517784351926253817694416293875897645132978532461631784259542169783",
        "283741596615239748974865321397126854861453972452978613528394167736512489149687235",
        "957638421146729385832541679419352768628417953375986142791265834583174296264893517",
        "127365489853491276964278351231756894548932617679184523312547968485619732796823145",
        "345671298987253146621984573264795831573816429198432657836529714712348965459167382",
        "265389471874251693193647852327894165946125387518763249631578924452916738789432516",
        "842359167573186942619274538127865394435791286968423715781942653354617829296538471",
        "538219746962874531174356298283497615741568329695123874329645187857931462416782953",
        "863751294957432681124689573532976148619843725748125936386217459295364817471598362",
        "986324157124759368537861429413285976695173284278946513342617895869532741751498632",
        "945671283136482597827593461614837952798125346253964178362759814581246739479318625",
        "724369851651248793893715642375691428912874365486523917238456179147982536569137284",
        "126437958895621473374985126457193862983246517612578394269314785548769231731852649",
        "152946837963587421847231695574863912289415763631729548796152384415398276328674159",
        "269314785548769231731852649126437958895621473374985126457193862983246517612578394",
        "152678943864391752973245681215763894497582136638914527321856479549127368786439215",
        "496573128381924675275861943153789462962435781847216539714352896529648317638197254",
        "253479861761238594894516732326791458915824376487653219548167923132985647679342185",
        "963741258152398674874265391345872169218956743697134825721489536589623417436517982",
        "152398647973641285864572931598714362247936158316285794725463819431829576689157423",
        "946731582157248639832659471719423865584976123623815947461397258398562714275184396",
        "932475861617928534845613279568741392429836715173259648356192487294387156781564923",
        "618459723342867519579123468296534187784291635153786294927648351861375942435912876",
        "957261384846537921123489567734926815295814736618375492572198643481653279369742158",
        "132749685857361924964285371216457839348692157579813246421536798683974512795128463",
        "746513892132869754598742316367925481925481673481637925679154238254378169813296547",
        "956327841127486395834951267548739612271864539369215478793548126415692783682173954",
        "935748621876231594124695783512469378643872915789153462267514839491386257358927146",
        "143258679872964153695137482986541327451372968237896514719623845564789231328415796",
        "937658241864291735125734986583419627649372518712586493471963852396825174258147369",
        "924361758156478293837592641613247985749185326582936174498623517371859462265714839",
        "856491372143572698927368451278645139514923786639817245361789524485236917792154863",
        "659412378238679451741385296865723149427891635913546782396157824574268913182934567",
        "354186927298743615167952483481527369932614578576398241729865134845231796613479852",
    ];

    use crate::{generator::SudokuGenerator, solver::SudokuSolver};

    fn sudoku_to_string(grid: &[usize]) -> String {
        return String::from_utf8(grid.iter().map(|c| (*c as f64).log2() as u8 + 48).collect())
            .unwrap();
    }

    #[test]
    fn test_solver() {
        let mut solver = SudokuSolver::new();
        for (test_case, solution) in zip(TEST_CASES, TEST_CASES_CORRECT) {
            let test_grid: Vec<usize> = test_case
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();

            let possible = solver.grid_to_possible(&test_grid);
            let result = solver.solve(&possible).unwrap();

            assert_eq!(sudoku_to_string(&result), solution);
        }
    }

    #[test]
    fn test_generator_valid_puzzle() {
        let mut generator = SudokuGenerator::new();
        let mut solver = SudokuSolver::new();

        for _ in 1..100 {
            let sudoku = generator.generate_sudoku();

            let possible = solver.grid_to_possible(&sudoku);
            assert!(solver.is_valid_puzzle(&possible));
        }
    }

    #[test]
    fn test_generator_correct_empty() {
        let mut generator = SudokuGenerator::new();

        for n in 55..60 {
            let sudoku = generator.generate_sudoku_with_empty(n);
            assert_eq!(sudoku.iter().filter(|&&c| c == 0).count(), n);
        }
    }

    #[test]
    fn test_generator_smallest_possible() {
        let mut generator = SudokuGenerator::new();
        let mut solver = SudokuSolver::new();

        for _ in 1..100 {
            let mut sudoku = generator.generate_sudoku();
            for i in 0..81 {
                let value = sudoku[i];
                if value == 0 {
                    continue;
                }
                sudoku[i] = 0;

                let possible = solver.grid_to_possible(&sudoku);
                assert!(!solver.is_valid_puzzle(&possible));
            }
        }
    }
}