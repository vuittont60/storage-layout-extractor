//! This module tests the library's analysis capabilities on the `UniswapV2Pair`
//! contract`.
#![cfg(test)]

use storage_layout_analyzer::{
    inference::abi::AbiType,
    layout::StorageSlot,
    watchdog::LazyWatchdog,
};

mod common;

/// Tests the analyser on the bytecode of the UniswapV2Pair contract deployed
/// [here](https://etherscan.io/address/0xCF6dAAB95c476106ECa715D48DE4b13287ffDEAa#code).
#[test]
fn correctly_generates_a_layout() -> anyhow::Result<()> {
    // Create the analyzer
    let bytecode = "0x608060405234801561001057600080fd5b50600436106101e55760003560e01c80637464fc3d1161010f578063c45a0155116100a2578063db1d0fd511610071578063db1d0fd5146105f4578063dd62ed3e146105fc578063fc061a4f1461062a578063fff6cae914610653576101e5565b8063c45a015514610551578063d13f90b414610559578063d21220a71461059b578063d505accf146105a3576101e5565b80639faa3c91116100de5780639faa3c91146104ef578063a9059cbb146104f7578063ba9a7a5614610523578063bc25cf771461052b576101e5565b80637464fc3d1461047a5780637ecebe001461048257806389afcb44146104a857806395d89b41146104e7576101e5565b806323b872dd116101875780635909c0d5116101565780635909c0d51461041e5780635a3d5493146104265780636a6278421461042e57806370a0823114610454576101e5565b806323b872dd146103ba57806330adf81f146103f0578063313ce567146103f85780633644e51514610416576101e5565b8063095ea7b3116101c3578063095ea7b3146103345780630dfe16811461037457806318160ddd146103985780631df4ccfc146103b2576101e5565b8063022c0d9f146101ea57806306fdde03146102785780630902f1ac146102f5575b600080fd5b6102766004803603608081101561020057600080fd5b8135916020810135916001600160a01b03604083013516919081019060808101606082013564010000000081111561023757600080fd5b82018360208201111561024957600080fd5b8035906020019184600183028401116401000000008311171561026b57600080fd5b50909250905061065b565b005b610280610bcd565b6040805160208082528351818301528351919283929083019185019080838360005b838110156102ba5781810151838201526020016102a2565b50505050905090810190601f1680156102e75780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6102fd610c06565b604080516dffffffffffffffffffffffffffff948516815292909316602083015263ffffffff168183015290519081900360600190f35b6103606004803603604081101561034a57600080fd5b506001600160a01b038135169060200135610c5b565b604080519115158252519081900360200190f35b61037c610c72565b604080516001600160a01b039092168252519081900360200190f35b6103a0610c81565b60408051918252519081900360200190f35b6103a0610c87565b610360600480360360608110156103d057600080fd5b506001600160a01b03813581169160208101359091169060400135610c8d565b6103a0610d3f565b610400610d63565b6040805160ff9092168252519081900360200190f35b6103a0610d68565b6103a0610d6e565b6103a0610d74565b6103a06004803603602081101561044457600080fd5b50356001600160a01b0316610d7a565b6103a06004803603602081101561046a57600080fd5b50356001600160a01b0316611276565b6103a0611288565b6103a06004803603602081101561049857600080fd5b50356001600160a01b031661128e565b6104ce600480360360208110156104be57600080fd5b50356001600160a01b03166112a0565b6040805192835260208301919091528051918290030190f35b610280611652565b6103a061168b565b6103606004803603604081101561050d57600080fd5b506001600160a01b038135169060200135611691565b6103a061169e565b6102766004803603602081101561054157600080fd5b50356001600160a01b03166116a4565b61037c611837565b610276600480360360a081101561056f57600080fd5b506001600160a01b03813581169160208101359091169060408101359060608101359060800135611846565b61037c6119cd565b610276600480360360e08110156105b957600080fd5b506001600160a01b03813581169160208101359091169060408101359060608101359060ff6080820135169060a08101359060c001356119dc565b6103a0611c04565b6103a06004803603604081101561061257600080fd5b506001600160a01b0381358116916020013516611c0a565b6102766004803603606081101561064057600080fd5b5080359060208101359060400135611c27565b610276611cda565b600f546001146106b2576040805162461bcd60e51b815260206004820152601160248201527f556e697377617056323a204c4f434b4544000000000000000000000000000000604482015290519081900360640190fd5b6000600f55841515806106c55750600084115b6107005760405162461bcd60e51b815260040180806020018281038252602581526020018061284f6025913960400191505060405180910390fd5b60008061070b610c06565b5091509150816dffffffffffffffffffffffffffff168710801561073e5750806dffffffffffffffffffffffffffff1686105b6107795760405162461bcd60e51b81526004018080602001828103825260218152602001806128986021913960400191505060405180910390fd5b60065460075460009182916001600160a01b039182169190811690891682148015906107b75750806001600160a01b0316896001600160a01b031614155b610808576040805162461bcd60e51b815260206004820152601560248201527f556e697377617056323a20494e56414c49445f544f0000000000000000000000604482015290519081900360640190fd5b8a1561081957610819828a8d611e5a565b891561082a5761082a818a8c611e5a565b86156108dc57886001600160a01b03166310d1e85c338d8d8c8c6040518663ffffffff1660e01b815260040180866001600160a01b03168152602001858152602001848152602001806020018281038252848482818152602001925080828437600081840152601f19601f8201169050808301925050509650505050505050600060405180830381600087803b1580156108c357600080fd5b505af11580156108d7573d6000803e3d6000fd5b505050505b604080516370a0823160e01b815230600482015290516001600160a01b038416916370a08231916024808301926020929190829003018186803b15801561092257600080fd5b505afa158015610936573d6000803e3d6000fd5b505050506040513d602081101561094c57600080fd5b5051604080516370a0823160e01b815230600482015290519195506001600160a01b038316916370a0823191602480820192602092909190829003018186803b15801561099857600080fd5b505afa1580156109ac573d6000803e3d6000fd5b505050506040513d60208110156109c257600080fd5b5051925060009150506dffffffffffffffffffffffffffff85168a900383116109ec576000610a02565b89856dffffffffffffffffffffffffffff160383035b9050600089856dffffffffffffffffffffffffffff16038311610a26576000610a3c565b89856dffffffffffffffffffffffffffff160383035b90506000821180610a4d5750600081115b610a885760405162461bcd60e51b81526004018080602001828103825260248152602001806128746024913960400191505060405180910390fd5b6000610ab4610aa2600c548561202290919063ffffffff16565b610aae876103e8612022565b9061208e565b90506000610ad0610aa2600c548561202290919063ffffffff16565b9050610afc620f4240610af66dffffffffffffffffffffffffffff8b8116908b16612022565b90612022565b610b068383612022565b1015610b59576040805162461bcd60e51b815260206004820152600c60248201527f556e697377617056323a204b0000000000000000000000000000000000000000604482015290519081900360640190fd5b5050610b67848488886120e6565b60408051838152602081018390528082018d9052606081018c905290516001600160a01b038b169133917fd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d8229181900360800190a350506001600f55505050505050505050565b6040518060400160405280601281526020017f536869626153776170204c5020546f6b656e000000000000000000000000000081525081565b6008546dffffffffffffffffffffffffffff808216926e0100000000000000000000000000008304909116917c0100000000000000000000000000000000000000000000000000000000900463ffffffff1690565b6000610c68338484612382565b5060015b92915050565b6006546001600160a01b031681565b60005481565b600c5481565b6001600160a01b03831660009081526002602090815260408083203384529091528120547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff14610d2a576001600160a01b0384166000908152600260209081526040808320338452909152902054610d05908361208e565b6001600160a01b03851660009081526002602090815260408083203384529091529020555b610d358484846123e4565b5060019392505050565b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c981565b601281565b60035481565b60095481565b600a5481565b6000600f54600114610dd3576040805162461bcd60e51b815260206004820152601160248201527f556e697377617056323a204c4f434b4544000000000000000000000000000000604482015290519081900360640190fd5b6000600f81905580610de3610c06565b50600654604080516370a0823160e01b815230600482015290519395509193506000926001600160a01b03909116916370a08231916024808301926020929190829003018186803b158015610e3757600080fd5b505afa158015610e4b573d6000803e3d6000fd5b505050506040513d6020811015610e6157600080fd5b5051600754604080516370a0823160e01b815230600482015290519293506000926001600160a01b03909216916370a0823191602480820192602092909190829003018186803b158015610eb457600080fd5b505afa158015610ec8573d6000803e3d6000fd5b505050506040513d6020811015610ede57600080fd5b505190506000610efe836dffffffffffffffffffffffffffff871661208e565b90506000610f1c836dffffffffffffffffffffffffffff871661208e565b90506000610f2a8787612492565b6000549091508061114757600554604080517f7cd07e4700000000000000000000000000000000000000000000000000000000815290516000926001600160a01b031691637cd07e47916004808301926020929190829003018186803b158015610f9357600080fd5b505afa158015610fa7573d6000803e3d6000fd5b505050506040513d6020811015610fbd57600080fd5b50519050336001600160a01b03821614156110be57806001600160a01b03166340dc0e376040518163ffffffff1660e01b815260040160206040518083038186803b15801561100b57600080fd5b505afa15801561101f573d6000803e3d6000fd5b505050506040513d602081101561103557600080fd5b50519950891580159061106857507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8a14155b6110b9576040805162461bcd60e51b815260206004820152601560248201527f4261642064657369726564206c69717569646974790000000000000000000000604482015290519081900360640190fd5b611141565b6001600160a01b0381161561111a576040805162461bcd60e51b815260206004820152601660248201527f4d757374206e6f742068617665206d69677261746f7200000000000000000000604482015290519081900360640190fd5b6111326103e8610aae61112d8888612022565b61260b565b995061114160006103e861265d565b50611198565b6111956dffffffffffffffffffffffffffff89166111658684612022565b8161116c57fe5b046dffffffffffffffffffffffffffff89166111888685612022565b8161118f57fe5b046126e7565b98505b600089116111d75760405162461bcd60e51b815260040180806020018281038252602881526020018061291b6028913960400191505060405180910390fd5b6111e18a8a61265d565b6111ed86868a8a6120e6565b811561122957600854611225906dffffffffffffffffffffffffffff808216916e010000000000000000000000000000900416612022565b600b555b6040805185815260208101859052815133927f4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f928290030190a250506001600f5550949695505050505050565b60016020526000908152604090205481565b600b5481565b60046020526000908152604090205481565b600080600f546001146112fa576040805162461bcd60e51b815260206004820152601160248201527f556e697377617056323a204c4f434b4544000000000000000000000000000000604482015290519081900360640190fd5b6000600f8190558061130a610c06565b50600654600754604080516370a0823160e01b815230600482015290519496509294506001600160a01b039182169391169160009184916370a08231916024808301926020929190829003018186803b15801561136657600080fd5b505afa15801561137a573d6000803e3d6000fd5b505050506040513d602081101561139057600080fd5b5051604080516370a0823160e01b815230600482015290519192506000916001600160a01b038516916370a08231916024808301926020929190829003018186803b1580156113de57600080fd5b505afa1580156113f2573d6000803e3d6000fd5b505050506040513d602081101561140857600080fd5b5051306000908152600160205260408120549192506114278888612492565b600054909150806114388487612022565b8161143f57fe5b049a508061144d8486612022565b8161145457fe5b04995060008b118015611467575060008a115b6114a25760405162461bcd60e51b81526004018080602001828103825260288152602001806128f36028913960400191505060405180910390fd5b6114ac30846126ff565b6114b7878d8d611e5a565b6114c2868d8c611e5a565b604080516370a0823160e01b815230600482015290516001600160a01b038916916370a08231916024808301926020929190829003018186803b15801561150857600080fd5b505afa15801561151c573d6000803e3d6000fd5b505050506040513d602081101561153257600080fd5b5051604080516370a0823160e01b815230600482015290519196506001600160a01b038816916370a0823191602480820192602092909190829003018186803b15801561157e57600080fd5b505afa158015611592573d6000803e3d6000fd5b505050506040513d60208110156115a857600080fd5b505193506115b885858b8b6120e6565b81156115f4576008546115f0906dffffffffffffffffffffffffffff808216916e010000000000000000000000000000900416612022565b600b555b604080518c8152602081018c905281516001600160a01b038f169233927fdccd412f0b1252819cb1fd330b93224ca42612892bb3f4f789976e6d81936496929081900390910190a35050505050505050506001600f81905550915091565b6040518060400160405280600481526020017f53534c500000000000000000000000000000000000000000000000000000000081525081565b600e5481565b6000610c683384846123e4565b6103e881565b600f546001146116fb576040805162461bcd60e51b815260206004820152601160248201527f556e697377617056323a204c4f434b4544000000000000000000000000000000604482015290519081900360640190fd5b6000600f55600654600754600854604080516370a0823160e01b815230600482015290516001600160a01b0394851694909316926117ab92859287926117a6926dffffffffffffffffffffffffffff169185916370a0823191602480820192602092909190829003018186803b15801561177457600080fd5b505afa158015611788573d6000803e3d6000fd5b505050506040513d602081101561179e57600080fd5b50519061208e565b611e5a565b61182d81846117a66008600e9054906101000a90046dffffffffffffffffffffffffffff166dffffffffffffffffffffffffffff16856001600160a01b03166370a08231306040518263ffffffff1660e01b815260040180826001600160a01b0316815260200191505060206040518083038186803b15801561177457600080fd5b50506001600f5550565b6005546001600160a01b031681565b6005546001600160a01b031633146118a5576040805162461bcd60e51b815260206004820152601460248201527f556e697377617056323a20464f5242494444454e000000000000000000000000604482015290519081900360640190fd5b600082116118fa576040805162461bcd60e51b815260206004820152601d60248201527f5f616c706861206d7573742062652067726561746572207468616e2030000000604482015290519081900360640190fd5b8181116119385760405162461bcd60e51b81526004018080602001828103825260268152602001806129436026913960400191505060405180910390fd5b600083116119775760405162461bcd60e51b815260040180806020018281038252603a8152602001806128b9603a913960400191505060405180910390fd5b600680546001600160a01b039687167fffffffffffffffffffffffff0000000000000000000000000000000000000000918216179091556007805495909616941693909317909355600c55600d91909155600e55565b6007546001600160a01b031681565b42841015611a31576040805162461bcd60e51b815260206004820152601260248201527f556e697377617056323a20455850495245440000000000000000000000000000604482015290519081900360640190fd5b6003546001600160a01b0380891660008181526004602090815260408083208054600180820190925582517f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98186015280840196909652958d166060860152608085018c905260a085019590955260c08085018b90528151808603909101815260e0850182528051908301207f19010000000000000000000000000000000000000000000000000000000000006101008601526101028501969096526101228085019690965280518085039096018652610142840180825286519683019690962095839052610162840180825286905260ff89166101828501526101a284018890526101c28401879052519193926101e280820193601f1981019281900390910190855afa158015611b67573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811615801590611b9d5750886001600160a01b0316816001600160a01b0316145b611bee576040805162461bcd60e51b815260206004820152601c60248201527f556e697377617056323a20494e56414c49445f5349474e415455524500000000604482015290519081900360640190fd5b611bf9898989612382565b505050505050505050565b600d5481565b600260209081526000928352604080842090915290825290205481565b6005546001600160a01b03163314611c86576040805162461bcd60e51b815260206004820152601460248201527f556e697377617056323a20464f5242494444454e000000000000000000000000604482015290519081900360640190fd5b600c839055600d829055600e819055604080518481526020810184905280820183905290517f509d432c4ab40e3eb039ee95fea93be8de6c751efa87aed5e51c7202b0dd8e099181900360600190a1505050565b600f54600114611d31576040805162461bcd60e51b815260206004820152601160248201527f556e697377617056323a204c4f434b4544000000000000000000000000000000604482015290519081900360640190fd5b6000600f55600654604080516370a0823160e01b81523060048201529051611e53926001600160a01b0316916370a08231916024808301926020929190829003018186803b158015611d8257600080fd5b505afa158015611d96573d6000803e3d6000fd5b505050506040513d6020811015611dac57600080fd5b5051600754604080516370a0823160e01b815230600482015290516001600160a01b03909216916370a0823191602480820192602092909190829003018186803b158015611df957600080fd5b505afa158015611e0d573d6000803e3d6000fd5b505050506040513d6020811015611e2357600080fd5b50516008546dffffffffffffffffffffffffffff808216916e0100000000000000000000000000009004166120e6565b6001600f55565b604080518082018252601981527f7472616e7366657228616464726573732c75696e74323536290000000000000060209182015281516001600160a01b0385811660248301526044808301869052845180840390910181526064909201845291810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fa9059cbb000000000000000000000000000000000000000000000000000000001781529251815160009460609489169392918291908083835b60208310611f355780518252601f199092019160209182019101611f16565b6001836020036101000a0380198251168184511680821785525050505050509050019150506000604051808303816000865af19150503d8060008114611f97576040519150601f19603f3d011682016040523d82523d6000602084013e611f9c565b606091505b5091509150818015611fca575080511580611fca5750808060200190516020811015611fc757600080fd5b50515b61201b576040805162461bcd60e51b815260206004820152601a60248201527f556e697377617056323a205452414e534645525f4641494c4544000000000000604482015290519081900360640190fd5b5050505050565b600081158061203d5750508082028282828161203a57fe5b04145b610c6c576040805162461bcd60e51b815260206004820152601460248201527f64732d6d6174682d6d756c2d6f766572666c6f77000000000000000000000000604482015290519081900360640190fd5b80820382811115610c6c576040805162461bcd60e51b815260206004820152601560248201527f64732d6d6174682d7375622d756e646572666c6f770000000000000000000000604482015290519081900360640190fd5b6dffffffffffffffffffffffffffff841180159061211257506dffffffffffffffffffffffffffff8311155b612163576040805162461bcd60e51b815260206004820152601360248201527f556e697377617056323a204f564552464c4f5700000000000000000000000000604482015290519081900360640190fd5b60085463ffffffff428116917c0100000000000000000000000000000000000000000000000000000000900481168203908116158015906121b357506dffffffffffffffffffffffffffff841615155b80156121ce57506dffffffffffffffffffffffffffff831615155b15612278578063ffffffff1661220b856121e786612791565b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff16906127b5565b600980547bffffffffffffffffffffffffffffffffffffffffffffffffffffffff929092169290920201905563ffffffff811661224b846121e787612791565b600a80547bffffffffffffffffffffffffffffffffffffffffffffffffffffffff92909216929092020190555b600880547fffffffffffffffffffffffffffffffffffff0000000000000000000000000000166dffffffffffffffffffffffffffff888116919091177fffffffff0000000000000000000000000000ffffffffffffffffffffffffffff166e0100000000000000000000000000008883168102919091177bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167c010000000000000000000000000000000000000000000000000000000063ffffffff871602179283905560408051848416815291909304909116602082015281517f1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1929181900390910190a1505050505050565b6001600160a01b03808416600081815260026020908152604080832094871680845294825291829020859055815185815291517f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259281900390910190a3505050565b6001600160a01b038316600090815260016020526040902054612407908261208e565b6001600160a01b03808516600090815260016020526040808220939093559084168152205461243690826127f6565b6001600160a01b0380841660008181526001602090815260409182902094909455805185815290519193928716927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef92918290030190a3505050565b600080600560009054906101000a90046001600160a01b03166001600160a01b031663017e7e586040518163ffffffff1660e01b815260040160206040518083038186803b1580156124e357600080fd5b505afa1580156124f7573d6000803e3d6000fd5b505050506040513d602081101561250d57600080fd5b5051600b546001600160a01b0382161580159450919250906125f75780156125f257600061255161112d6dffffffffffffffffffffffffffff888116908816612022565b9050600061255e8361260b565b9050808211156125ef57600d5460009061258890610af661257f868661208e565b60005490612022565b905060006125cb6125a4600d548561202290919063ffffffff16565b6125c56125be600d54600e5461208e90919063ffffffff16565b8790612022565b906127f6565b905060008183816125d857fe5b04905080156125eb576125eb878261265d565b5050505b50505b612603565b8015612603576000600b555b505092915050565b6000600382111561264e575080600160028204015b818110156126485780915060028182858161263757fe5b04018161264057fe5b049050612620565b50612658565b8115612658575060015b919050565b60005461266a90826127f6565b60009081556001600160a01b03831681526001602052604090205461268f90826127f6565b6001600160a01b03831660008181526001602090815260408083209490945583518581529351929391927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9281900390910190a35050565b60008183106126f657816126f8565b825b9392505050565b6001600160a01b038216600090815260016020526040902054612722908261208e565b6001600160a01b03831660009081526001602052604081209190915554612749908261208e565b60009081556040805183815290516001600160a01b038516917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef919081900360200190a35050565b6dffffffffffffffffffffffffffff166e0100000000000000000000000000000290565b60006dffffffffffffffffffffffffffff82167bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8416816127ee57fe5b049392505050565b80820182811015610c6c576040805162461bcd60e51b815260206004820152601460248201527f64732d6d6174682d6164642d6f766572666c6f77000000000000000000000000604482015290519081900360640190fdfe556e697377617056323a20494e53554646494349454e545f4f55545055545f414d4f554e54556e697377617056323a20494e53554646494349454e545f494e5055545f414d4f554e54556e697377617056323a20494e53554646494349454e545f4c4951554944495459746f74616c4665652073686f756c64206e6f7420626520302c2077686963682077696c6c20616c6c6f77206672656520666c6173682073776170556e697377617056323a20494e53554646494349454e545f4c49515549444954595f4255524e4544556e697377617056323a20494e53554646494349454e545f4c49515549444954595f4d494e544544626574612073686f756c6420616c77617973206265206c61746572207468616e20616c706861a2646970667358221220b562054fd180879e709cb9f95feb3c261a153a823344d5c22c2f0cda0c56df5d64736f6c634300060c0033";
    let analyzer = common::new_analyzer_from_bytecode(bytecode, LazyWatchdog.in_rc())?;

    // Get the final storage layout for the input contract
    let layout = analyzer.analyze()?;

    // We should see 18 entries as we output packing in slots as separate slots, but
    // we only see 16 due to not seeing part of a packed.
    assert_eq!(layout.slots().len(), 16);

    // For the ones we can infer to a non-conflict, let's make sure we keep getting
    // them 'right'

    // `uint256` but we infer `uintUnknown`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(0, 0, AbiType::UInt { size: None }))
    );

    // `mapping(address => uintUnknown)`
    assert!(layout.slots().contains(&StorageSlot::new(
        1,
        0,
        AbiType::Mapping {
            key_type:   Box::new(AbiType::Address),
            value_type: Box::new(AbiType::UInt { size: None }),
        },
    )));

    // `mapping(address => mapping(address => uint256))`
    assert!(layout.slots().contains(&StorageSlot::new(
        2,
        0,
        AbiType::Mapping {
            key_type:   Box::new(AbiType::Address),
            value_type: Box::new(AbiType::Mapping {
                key_type:   Box::new(AbiType::Address),
                value_type: Box::new(AbiType::UInt { size: Some(256) }),
            }),
        }
    )));

    // `bytes32` but we infer `any`
    assert!(layout.slots().contains(&StorageSlot::new(3, 0, AbiType::Any)));

    // `mapping(address => number)`
    assert!(layout.slots().contains(&StorageSlot::new(
        4,
        0,
        AbiType::Mapping {
            key_type:   Box::new(AbiType::Address),
            value_type: Box::new(AbiType::Number { size: None }),
        }
    )));

    // `address` but we infer `conflict`
    assert_eq!(layout.slots()[5].index, 5);
    assert_eq!(layout.slots()[5].offset, 0);
    assert!(matches!(
        &layout.slots()[5].typ,
        AbiType::ConflictedType { .. }
    ));

    // `address`
    assert!(layout.slots().contains(&StorageSlot::new(6, 0, AbiType::Address)));

    // `address`
    assert!(layout.slots().contains(&StorageSlot::new(7, 0, AbiType::Address)));

    // `packed(uint112, uint112)`, but we infer `conflict`
    assert_eq!(layout.slots()[8].index, 8);
    assert_eq!(layout.slots()[8].offset, 0);
    assert!(matches!(
        &layout.slots()[8].typ,
        AbiType::ConflictedType { .. }
    ));

    // `uint32`, but we miss it entirely
    assert!(!layout.slots().iter().any(|s| s.index == 8 && s.offset == 224));

    // `uint256`, but we infer `number`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(9, 0, AbiType::Number { size: None }))
    );

    // `uint256`, but we infer `number`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(10, 0, AbiType::Number { size: None }))
    );

    // `uint256`, but we infer `uintUnknown`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(11, 0, AbiType::UInt { size: None }))
    );

    // `uint256`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(12, 0, AbiType::UInt { size: Some(256) }))
    );

    // `uint256`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(13, 0, AbiType::UInt { size: Some(256) }))
    );

    // `uint256`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(14, 0, AbiType::UInt { size: Some(256) }))
    );

    // `uint256`, but we infer `bytesUnknown`
    assert!(
        layout
            .slots()
            .contains(&StorageSlot::new(15, 0, AbiType::Bytes { length: None }))
    );

    Ok(())
}
