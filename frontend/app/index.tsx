import {FlatList, Pressable, StyleSheet, Text, View} from "react-native";
import {useEffect, useState} from "react";

export default function Index() {
    const [dockers, setDockers] = useState<DockerContainer[]>([]);
    const [dataLoading, setDataLoading] = useState<boolean>(false);

    useEffect(() => {
        const getData = () => {
            fetch('http://127.0.0.1:8000/sudo/docker/ps')
                .then(response => response.text())
                .then(json => {
                    console.log(json);
                    const jsonValue: DockerContainer[] = JSON.parse(json);

                    return jsonValue;
                })
                .then(dockers => {
                    setDockers(dockers);
                    console.log(dockers);
                })
                .catch((error) => {
                    console.log(error);
                });
        }
        getData();
    }, [dataLoading])
    
    return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
        <FlatList 
            data={dockers} 
            keyExtractor={({id}) => id} 
            renderItem={({item}) => (
              <View style={styles.listView}>
                  <Text style={styles.text}>name: {item.name}</Text>
                  <Text style={styles.text}>id: {item.id}</Text>
                  <Text style={styles.text}>image: {item.image}</Text>
                  <Text style={styles.text}>command: {item.command}</Text>
                  <Text style={styles.text}>created: {item.created}</Text>
                  <Text style={styles.text}>status: {item.status}</Text>
              </View>
            )}
        />
        <Pressable onPress={() => {setDataLoading((loading) => !loading)}}>
            <Text style={styles.text}>Refetch!</Text>
        </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
    text: {
        fontSize: 20,
        fontWeight: "bold",
    },
    listView: {
        flex: 1,
        justifyContent: "space-between",
        alignItems: "center",
    }
});

type IDockerContainer = {
    id: string,
    image: string,
    command: string,
    created: string,
    status: string,
    ports: string[],
    name: string
}

class DockerContainer implements IDockerContainer {
    id: string;
    image: string;
    command: string;
    created: string;
    status: string;
    ports: string[];
    name: string;
    
    constructor(id: string, image: string, command: string, created: string, status: string, ports: string[], name: string) {
        this.id = id;
        this.image = image;
        this.command = command;
        this.created = created;
        this.status = status;
        this.ports = ports;
        this.name = name;
    }
}