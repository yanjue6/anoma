(function() {var implementors = {};
implementors["anoma"] = [{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/config/enum.Error.html\" title=\"enum anoma::config::Error\">Error</a>","synthetic":true,"types":["anoma::config::Error"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/config/struct.Ledger.html\" title=\"struct anoma::config::Ledger\">Ledger</a>","synthetic":true,"types":["anoma::config::Ledger"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/config/struct.RpcServer.html\" title=\"struct anoma::config::RpcServer\">RpcServer</a>","synthetic":true,"types":["anoma::config::RpcServer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/config/struct.Matchmaker.html\" title=\"struct anoma::config::Matchmaker\">Matchmaker</a>","synthetic":true,"types":["anoma::config::Matchmaker"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/config/enum.SubscriptionFilter.html\" title=\"enum anoma::config::SubscriptionFilter\">SubscriptionFilter</a>","synthetic":true,"types":["anoma::config::SubscriptionFilter"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/config/struct.IntentBroadcaster.html\" title=\"struct anoma::config::IntentBroadcaster\">IntentBroadcaster</a>","synthetic":true,"types":["anoma::config::IntentBroadcaster"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/config/struct.Config.html\" title=\"struct anoma::config::Config\">Config</a>","synthetic":true,"types":["anoma::config::Config"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/genesis/struct.Genesis.html\" title=\"struct anoma::genesis::Genesis\">Genesis</a>","synthetic":true,"types":["anoma::genesis::Genesis"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/genesis/struct.Validator.html\" title=\"struct anoma::genesis::Validator\">Validator</a>","synthetic":true,"types":["anoma::genesis::Validator"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/gossiper/struct.Gossiper.html\" title=\"struct anoma::gossiper::Gossiper\">Gossiper</a>","synthetic":true,"types":["anoma::gossiper::Gossiper"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/gossip/enum.Error.html\" title=\"enum anoma::node::gossip::Error\">Error</a>","synthetic":true,"types":["anoma::node::gossip::Error"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/protocol/enum.Error.html\" title=\"enum anoma::node::protocol::Error\">Error</a>","synthetic":true,"types":["anoma::node::protocol::Error"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/protocol/struct.TxResult.html\" title=\"struct anoma::node::protocol::TxResult\">TxResult</a>","synthetic":true,"types":["anoma::node::protocol::TxResult"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/protocol/struct.VpsResult.html\" title=\"struct anoma::node::protocol::VpsResult\">VpsResult</a>","synthetic":true,"types":["anoma::node::protocol::VpsResult"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/shell/gas/enum.Error.html\" title=\"enum anoma::node::shell::gas::Error\">Error</a>","synthetic":true,"types":["anoma::node::shell::gas::Error"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/shell/gas/struct.BlockGasMeter.html\" title=\"struct anoma::node::shell::gas::BlockGasMeter\">BlockGasMeter</a>","synthetic":true,"types":["anoma::node::shell::gas::BlockGasMeter"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/shell/gas/struct.VpGasMeter.html\" title=\"struct anoma::node::shell::gas::VpGasMeter\">VpGasMeter</a>","synthetic":true,"types":["anoma::node::shell::gas::VpGasMeter"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/shell/gas/struct.VpsGas.html\" title=\"struct anoma::node::shell::gas::VpsGas\">VpsGas</a>","synthetic":true,"types":["anoma::node::shell::gas::VpsGas"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/shell/storage/enum.Error.html\" title=\"enum anoma::node::shell::storage::Error\">Error</a>","synthetic":true,"types":["anoma::node::shell::storage::Error"]},{"text":"impl&lt;DB&gt; Freeze for <a class=\"struct\" href=\"anoma/node/shell/storage/struct.Storage.html\" title=\"struct anoma::node::shell::storage::Storage\">Storage</a>&lt;DB&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: Freeze,&nbsp;</span>","synthetic":true,"types":["anoma::node::shell::storage::Storage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/shell/storage/struct.BlockStorage.html\" title=\"struct anoma::node::shell::storage::BlockStorage\">BlockStorage</a>","synthetic":true,"types":["anoma::node::shell::storage::BlockStorage"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/shell/enum.Error.html\" title=\"enum anoma::node::shell::Error\">Error</a>","synthetic":true,"types":["anoma::node::shell::Error"]},{"text":"impl !Freeze for <a class=\"struct\" href=\"anoma/node/shell/struct.Shell.html\" title=\"struct anoma::node::shell::Shell\">Shell</a>","synthetic":true,"types":["anoma::node::shell::Shell"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/shell/enum.MempoolTxType.html\" title=\"enum anoma::node::shell::MempoolTxType\">MempoolTxType</a>","synthetic":true,"types":["anoma::node::shell::MempoolTxType"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/shell/struct.MerkleRoot.html\" title=\"struct anoma::node::shell::MerkleRoot\">MerkleRoot</a>","synthetic":true,"types":["anoma::node::shell::MerkleRoot"]},{"text":"impl&lt;'iter, DB&gt; Freeze for <a class=\"struct\" href=\"anoma/node/vm/host_env/prefix_iter/struct.PrefixIterators.html\" title=\"struct anoma::node::vm::host_env::prefix_iter::PrefixIterators\">PrefixIterators</a>&lt;'iter, DB&gt;","synthetic":true,"types":["anoma::node::vm::host_env::prefix_iter::PrefixIterators"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/host_env/prefix_iter/struct.PrefixIteratorId.html\" title=\"struct anoma::node::vm::host_env::prefix_iter::PrefixIteratorId\">PrefixIteratorId</a>","synthetic":true,"types":["anoma::node::vm::host_env::prefix_iter::PrefixIteratorId"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/vm/host_env/write_log/enum.Error.html\" title=\"enum anoma::node::vm::host_env::write_log::Error\">Error</a>","synthetic":true,"types":["anoma::node::vm::host_env::write_log::Error"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/vm/host_env/write_log/enum.StorageModification.html\" title=\"enum anoma::node::vm::host_env::write_log::StorageModification\">StorageModification</a>","synthetic":true,"types":["anoma::node::vm::host_env::write_log::StorageModification"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/host_env/write_log/struct.WriteLog.html\" title=\"struct anoma::node::vm::host_env::write_log::WriteLog\">WriteLog</a>","synthetic":true,"types":["anoma::node::vm::host_env::write_log::WriteLog"]},{"text":"impl&lt;'a, DB&gt; Freeze for <a class=\"struct\" href=\"anoma/node/vm/host_env/struct.VpEnv.html\" title=\"struct anoma::node::vm::host_env::VpEnv\">VpEnv</a>&lt;'a, DB&gt;","synthetic":true,"types":["anoma::node::vm::host_env::VpEnv"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/host_env/struct.MatchmakerEnv.html\" title=\"struct anoma::node::vm::host_env::MatchmakerEnv\">MatchmakerEnv</a>","synthetic":true,"types":["anoma::node::vm::host_env::MatchmakerEnv"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/host_env/struct.FilterEnv.html\" title=\"struct anoma::node::vm::host_env::FilterEnv\">FilterEnv</a>","synthetic":true,"types":["anoma::node::vm::host_env::FilterEnv"]},{"text":"impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.EnvHostWrapper.html\" title=\"struct anoma::node::vm::EnvHostWrapper\">EnvHostWrapper</a>&lt;'a, T&gt;","synthetic":true,"types":["anoma::node::vm::EnvHostWrapper"]},{"text":"impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.EnvHostSliceWrapper.html\" title=\"struct anoma::node::vm::EnvHostSliceWrapper\">EnvHostSliceWrapper</a>&lt;'a, T&gt;","synthetic":true,"types":["anoma::node::vm::EnvHostSliceWrapper"]},{"text":"impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.MutEnvHostWrapper.html\" title=\"struct anoma::node::vm::MutEnvHostWrapper\">MutEnvHostWrapper</a>&lt;'a, T&gt;","synthetic":true,"types":["anoma::node::vm::MutEnvHostWrapper"]},{"text":"impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.MutEnvHostSliceWrapper.html\" title=\"struct anoma::node::vm::MutEnvHostSliceWrapper\">MutEnvHostSliceWrapper</a>&lt;'a, T&gt;","synthetic":true,"types":["anoma::node::vm::MutEnvHostSliceWrapper"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.TxRunner.html\" title=\"struct anoma::node::vm::TxRunner\">TxRunner</a>","synthetic":true,"types":["anoma::node::vm::TxRunner"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/node/vm/enum.Error.html\" title=\"enum anoma::node::vm::Error\">Error</a>","synthetic":true,"types":["anoma::node::vm::Error"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.VpRunner.html\" title=\"struct anoma::node::vm::VpRunner\">VpRunner</a>","synthetic":true,"types":["anoma::node::vm::VpRunner"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.MatchmakerRunner.html\" title=\"struct anoma::node::vm::MatchmakerRunner\">MatchmakerRunner</a>","synthetic":true,"types":["anoma::node::vm::MatchmakerRunner"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/node/vm/struct.FilterRunner.html\" title=\"struct anoma::node::vm::FilterRunner\">FilterRunner</a>","synthetic":true,"types":["anoma::node::vm::FilterRunner"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/proto/services/rpc_message/enum.Message.html\" title=\"enum anoma::proto::services::rpc_message::Message\">Message</a>","synthetic":true,"types":["anoma::proto::generated::services::rpc_message::Message"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"anoma/proto/services/rpc_service_client/struct.RpcServiceClient.html\" title=\"struct anoma::proto::services::rpc_service_client::RpcServiceClient\">RpcServiceClient</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":["anoma::proto::generated::services::rpc_service_client::RpcServiceClient"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"anoma/proto/services/rpc_service_server/struct.RpcServiceServer.html\" title=\"struct anoma::proto::services::rpc_service_server::RpcServiceServer\">RpcServiceServer</a>&lt;T&gt;","synthetic":true,"types":["anoma::proto::generated::services::rpc_service_server::RpcServiceServer"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/services/struct.IntentMesage.html\" title=\"struct anoma::proto::services::IntentMesage\">IntentMesage</a>","synthetic":true,"types":["anoma::proto::generated::services::IntentMesage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/services/struct.SubscribeTopicMessage.html\" title=\"struct anoma::proto::services::SubscribeTopicMessage\">SubscribeTopicMessage</a>","synthetic":true,"types":["anoma::proto::generated::services::SubscribeTopicMessage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/services/struct.RpcMessage.html\" title=\"struct anoma::proto::services::RpcMessage\">RpcMessage</a>","synthetic":true,"types":["anoma::proto::generated::services::RpcMessage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/services/struct.RpcResponse.html\" title=\"struct anoma::proto::services::RpcResponse\">RpcResponse</a>","synthetic":true,"types":["anoma::proto::generated::services::RpcResponse"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/proto/types/intent_broadcaster_message/enum.Msg.html\" title=\"enum anoma::proto::types::intent_broadcaster_message::Msg\">Msg</a>","synthetic":true,"types":["anoma::proto::generated::types::intent_broadcaster_message::Msg"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/proto/types/dkg_broadcaster_message/enum.DkgMessage.html\" title=\"enum anoma::proto::types::dkg_broadcaster_message::DkgMessage\">DkgMessage</a>","synthetic":true,"types":["anoma::proto::generated::types::dkg_broadcaster_message::DkgMessage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/types/struct.Tx.html\" title=\"struct anoma::proto::types::Tx\">Tx</a>","synthetic":true,"types":["anoma::proto::generated::types::Tx"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/types/struct.Intent.html\" title=\"struct anoma::proto::types::Intent\">Intent</a>","synthetic":true,"types":["anoma::proto::generated::types::Intent"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/types/struct.IntentBroadcasterMessage.html\" title=\"struct anoma::proto::types::IntentBroadcasterMessage\">IntentBroadcasterMessage</a>","synthetic":true,"types":["anoma::proto::generated::types::IntentBroadcasterMessage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/types/struct.Dkg.html\" title=\"struct anoma::proto::types::Dkg\">Dkg</a>","synthetic":true,"types":["anoma::proto::generated::types::Dkg"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/types/struct.DkgBroadcasterMessage.html\" title=\"struct anoma::proto::types::DkgBroadcasterMessage\">DkgBroadcasterMessage</a>","synthetic":true,"types":["anoma::proto::generated::types::DkgBroadcasterMessage"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma/proto/struct.IntentId.html\" title=\"struct anoma::proto::IntentId\">IntentId</a>","synthetic":true,"types":["anoma::proto::IntentId"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma/types/enum.MatchmakerMessage.html\" title=\"enum anoma::types::MatchmakerMessage\">MatchmakerMessage</a>","synthetic":true,"types":["anoma::types::MatchmakerMessage"]}];
implementors["anoma_shared"] = [{"text":"impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"anoma_shared/bytes/struct.ByteBuf.html\" title=\"struct anoma_shared::bytes::ByteBuf\">ByteBuf</a>&lt;'a&gt;","synthetic":true,"types":["anoma_shared::bytes::ByteBuf"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/address/enum.Error.html\" title=\"enum anoma_shared::types::address::Error\">Error</a>","synthetic":true,"types":["anoma_shared::types::address::Error"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/address/enum.Address.html\" title=\"enum anoma_shared::types::address::Address\">Address</a>","synthetic":true,"types":["anoma_shared::types::address::Address"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/address/struct.EstablishedAddress.html\" title=\"struct anoma_shared::types::address::EstablishedAddress\">EstablishedAddress</a>","synthetic":true,"types":["anoma_shared::types::address::EstablishedAddress"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/address/struct.EstablishedAddressGen.html\" title=\"struct anoma_shared::types::address::EstablishedAddressGen\">EstablishedAddressGen</a>","synthetic":true,"types":["anoma_shared::types::address::EstablishedAddressGen"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/address/enum.ImplicitAddress.html\" title=\"enum anoma_shared::types::address::ImplicitAddress\">ImplicitAddress</a>","synthetic":true,"types":["anoma_shared::types::address::ImplicitAddress"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/intent/struct.Intent.html\" title=\"struct anoma_shared::types::intent::Intent\">Intent</a>","synthetic":true,"types":["anoma_shared::types::intent::Intent"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/intent/struct.IntentTransfers.html\" title=\"struct anoma_shared::types::intent::IntentTransfers\">IntentTransfers</a>","synthetic":true,"types":["anoma_shared::types::intent::IntentTransfers"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/internal/enum.HostEnvResult.html\" title=\"enum anoma_shared::types::internal::HostEnvResult\">HostEnvResult</a>","synthetic":true,"types":["anoma_shared::types::internal::HostEnvResult"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.PublicKey.html\" title=\"struct anoma_shared::types::key::ed25519::PublicKey\">PublicKey</a>","synthetic":true,"types":["anoma_shared::types::key::ed25519::PublicKey"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signature.html\" title=\"struct anoma_shared::types::key::ed25519::Signature\">Signature</a>","synthetic":true,"types":["anoma_shared::types::key::ed25519::Signature"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.PublicKeyHash.html\" title=\"struct anoma_shared::types::key::ed25519::PublicKeyHash\">PublicKeyHash</a>","synthetic":true,"types":["anoma_shared::types::key::ed25519::PublicKeyHash"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/key/ed25519/enum.VerifySigError.html\" title=\"enum anoma_shared::types::key::ed25519::VerifySigError\">VerifySigError</a>","synthetic":true,"types":["anoma_shared::types::key::ed25519::VerifySigError"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.SignedTxData.html\" title=\"struct anoma_shared::types::key::ed25519::SignedTxData\">SignedTxData</a>","synthetic":true,"types":["anoma_shared::types::key::ed25519::SignedTxData"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"anoma_shared/types/key/ed25519/struct.Signed.html\" title=\"struct anoma_shared::types::key::ed25519::Signed\">Signed</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":["anoma_shared::types::key::ed25519::Signed"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/token/struct.Amount.html\" title=\"struct anoma_shared::types::token::Amount\">Amount</a>","synthetic":true,"types":["anoma_shared::types::token::Amount"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/token/struct.Transfer.html\" title=\"struct anoma_shared::types::token::Transfer\">Transfer</a>","synthetic":true,"types":["anoma_shared::types::token::Transfer"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/enum.Error.html\" title=\"enum anoma_shared::types::Error\">Error</a>","synthetic":true,"types":["anoma_shared::types::Error"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/struct.BlockHeight.html\" title=\"struct anoma_shared::types::BlockHeight\">BlockHeight</a>","synthetic":true,"types":["anoma_shared::types::BlockHeight"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/struct.BlockHash.html\" title=\"struct anoma_shared::types::BlockHash\">BlockHash</a>","synthetic":true,"types":["anoma_shared::types::BlockHash"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/struct.Key.html\" title=\"struct anoma_shared::types::Key\">Key</a>","synthetic":true,"types":["anoma_shared::types::Key"]},{"text":"impl Freeze for <a class=\"enum\" href=\"anoma_shared/types/enum.DbKeySeg.html\" title=\"enum anoma_shared::types::DbKeySeg\">DbKeySeg</a>","synthetic":true,"types":["anoma_shared::types::DbKeySeg"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/types/struct.UpdateVp.html\" title=\"struct anoma_shared::types::UpdateVp\">UpdateVp</a>","synthetic":true,"types":["anoma_shared::types::UpdateVp"]},{"text":"impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.VpInput.html\" title=\"struct anoma_shared::vm_memory::VpInput\">VpInput</a>&lt;'a&gt;","synthetic":true,"types":["anoma_shared::vm_memory::VpInput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.KeyVal.html\" title=\"struct anoma_shared::vm_memory::KeyVal\">KeyVal</a>","synthetic":true,"types":["anoma_shared::vm_memory::KeyVal"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageReadInput.html\" title=\"struct anoma_shared::vm_memory::StorageReadInput\">StorageReadInput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageReadInput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageReadOutput.html\" title=\"struct anoma_shared::vm_memory::StorageReadOutput\">StorageReadOutput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageReadOutput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageHasKeyInput.html\" title=\"struct anoma_shared::vm_memory::StorageHasKeyInput\">StorageHasKeyInput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageHasKeyInput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageHasKeyOutput.html\" title=\"struct anoma_shared::vm_memory::StorageHasKeyOutput\">StorageHasKeyOutput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageHasKeyOutput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageWriteInput.html\" title=\"struct anoma_shared::vm_memory::StorageWriteInput\">StorageWriteInput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageWriteInput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageReadSelfInput.html\" title=\"struct anoma_shared::vm_memory::StorageReadSelfInput\">StorageReadSelfInput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageReadSelfInput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.StorageReadSelfOutput.html\" title=\"struct anoma_shared::vm_memory::StorageReadSelfOutput\">StorageReadSelfOutput</a>","synthetic":true,"types":["anoma_shared::vm_memory::StorageReadSelfOutput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.OtherApprovedInput.html\" title=\"struct anoma_shared::vm_memory::OtherApprovedInput\">OtherApprovedInput</a>","synthetic":true,"types":["anoma_shared::vm_memory::OtherApprovedInput"]},{"text":"impl Freeze for <a class=\"struct\" href=\"anoma_shared/vm_memory/struct.OtherApprovedOutput.html\" title=\"struct anoma_shared::vm_memory::OtherApprovedOutput\">OtherApprovedOutput</a>","synthetic":true,"types":["anoma_shared::vm_memory::OtherApprovedOutput"]}];
implementors["anoma_vm_env"] = [{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"anoma_vm_env/tx_prelude/struct.KeyValIterator.html\" title=\"struct anoma_vm_env::tx_prelude::KeyValIterator\">KeyValIterator</a>&lt;T&gt;","synthetic":true,"types":["anoma_vm_env::imports::tx::KeyValIterator"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"anoma_vm_env/vp_prelude/struct.PreKeyValIterator.html\" title=\"struct anoma_vm_env::vp_prelude::PreKeyValIterator\">PreKeyValIterator</a>&lt;T&gt;","synthetic":true,"types":["anoma_vm_env::imports::vp::PreKeyValIterator"]},{"text":"impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"anoma_vm_env/vp_prelude/struct.PostKeyValIterator.html\" title=\"struct anoma_vm_env::vp_prelude::PostKeyValIterator\">PostKeyValIterator</a>&lt;T&gt;","synthetic":true,"types":["anoma_vm_env::imports::vp::PostKeyValIterator"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()