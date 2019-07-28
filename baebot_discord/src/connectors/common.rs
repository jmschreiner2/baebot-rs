pub enum ConnectorResponseType
{
    Uri = 0b00000001,
    Attachment = 0b00000010
};

struct ConnectorResponse
{
    uri: String,
    stream: String,
    type: C
}

trait Connector 
{
    fn connect(&self, Vec<String>) -> ConnectorResponse;
}