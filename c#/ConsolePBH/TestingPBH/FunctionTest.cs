using Microsoft.Extensions.Options;
using NUnit.Framework;
using PasswordBasedHashing;

namespace TestingPBH
{
    public class FunctionTest
    {

        public HashEncryptionService HashEncryptionService { get; set; }

        [SetUp]
        public void Setup()
        {
            IOptions<HashingOptions> hashingOptions = new OptionsWrapper<HashingOptions>(new HashingOptions(10000));
            HashEncryptionService = new HashEncryptionService(hashingOptions);
        }

        [Test]
        public void TestHashFormat()
        {
            string input = "anExamplePassword123!";

            string hash = HashEncryptionService.Hash(input);

            string[] parts = hash.Split('.', 3);
            bool correctFormat = parts.Length == 3;
            Assert.True(correctFormat, "Unexpected hash format. Should be formatted as [iterations].[salt].[hash]");
        }

        [Test]
        public void TestHashCheckVerified()
        {
            string input = "anExamplePassword123!";
            string hash = HashEncryptionService.Hash(input);

            (bool Verified, bool NeedsUpgrade) checkResults = HashEncryptionService.Check(hash, input);
            Assert.True(checkResults.Verified, "Hash check should have returned verified, but did not.");
        }

        [Test]
        public void TestHashCheckNotVerified()
        {
            const string input = "anExamplePassword123!";
            const string badInput = "anExamplePassword123?";
            string hash = HashEncryptionService.Hash(input);

            (bool Verified, bool NeedsUpgrade) checkResults = HashEncryptionService.Check(hash, badInput);
            Assert.IsFalse(checkResults.Verified, "Hash check should have been unverified, but was verified.");
        }

    }
}