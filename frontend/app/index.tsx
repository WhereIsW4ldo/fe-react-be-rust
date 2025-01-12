import {Pressable, ScrollView, StyleSheet, Text, View} from "react-native";
import {useEffect, useState} from "react";
import { Table, Row, Rows } from 'react-native-table-component';

export default function Index() {
    const [dockers, setDockers] = useState<DockerContainer[]>([]);
    const [dataLoading, setDataLoading] = useState<boolean>(false);
    
    const headers = ['name', 'id', 'image', 'command', 'created', 'status'];

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

    useEffect(() => {
        const timer = setInterval(() => getData(), 1000);
        return () => clearInterval(timer);
    }, [])
    
    return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
        <ScrollView style={styles.container}>
            <Table borderStyle={{borderWidth: 2}}>
                <Row data={headers} style={styles.head} textStyle={styles.text}/>
                <Rows data={dockers.map((x) => {return [x.Names, x.ID, x.Image, x.Command, x.CreatedAt, x.Status];})} textStyle={styles.text}/>
            </Table>
        </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
    container: { padding: 16, paddingTop: 30, width: 1000},
    head: { height: 40, backgroundColor: '#f1f8ff' },
    text: { margin: 6 }
});

type IDockerContainer = {
    ID: string,
    Image: string,
    Command: string,
    CreatedAt: string,
    Status: string,
    Ports: string,
    Names: string
}

class DockerContainer implements IDockerContainer {
    ID: string;
    Image: string;
    Command: string;
    CreatedAt: string;
    Status: string;
    Ports: string;
    Names: string;
    
    constructor(id: string, image: string, command: string, created: string, status: string, ports: string, name: string) {
        this.ID = id;
        this.Image = image;
        this.Command = command;
        this.CreatedAt = created;
        this.Status = status;
        this.Ports = ports;
        this.Names = name;
    }
}